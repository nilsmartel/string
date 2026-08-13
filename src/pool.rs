use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;

/// Run `job` for every item, on up to `threads` worker threads.
///
/// `on_result` is invoked on the *calling* thread, so it may hold on to things that aren't `Send`,
/// like the output stream. Results are handed to it in the order the jobs complete, or, when
/// `ordered` is set, in the order of `items` — which means a result has to wait in memory until
/// every earlier item finished.
pub fn for_each<T, R>(
    items: &[T],
    threads: usize,
    ordered: bool,
    job: impl Fn(&T) -> R + Sync,
    mut on_result: impl FnMut(usize, R) -> anyhow::Result<()>,
) -> anyhow::Result<()>
where
    T: Sync,
    R: Send,
{
    if items.is_empty() {
        return Ok(());
    }

    let threads = threads.max(1).min(items.len());

    // the next item to be picked up. Handing out one item at a time keeps all workers busy,
    // even when some items take much longer than others.
    let cursor = AtomicUsize::new(0);
    let cancelled = AtomicBool::new(false);
    let (sender, receiver) = mpsc::channel::<(usize, R)>();

    std::thread::scope(|scope| {
        for _ in 0..threads {
            let sender = sender.clone();
            let cursor = &cursor;
            let cancelled = &cancelled;
            let job = &job;

            scope.spawn(move || loop {
                if cancelled.load(Ordering::Relaxed) {
                    break;
                }

                let index = cursor.fetch_add(1, Ordering::Relaxed);
                if index >= items.len() {
                    break;
                }

                if sender.send((index, job(&items[index]))).is_err() {
                    break;
                }
            });
        }

        // the workers hold their own senders. Dropping ours lets the loop below end
        // as soon as the last worker is done.
        drop(sender);

        let mut pending = BTreeMap::new();
        let mut next = 0;

        for (index, value) in receiver {
            let outcome = if ordered {
                pending.insert(index, value);

                let mut outcome = Ok(());
                while let Some(value) = pending.remove(&next) {
                    outcome = on_result(next, value);
                    next += 1;

                    if outcome.is_err() {
                        break;
                    }
                }
                outcome
            } else {
                on_result(index, value)
            };

            if let Err(e) = outcome {
                // nothing consumes results anymore, so stop handing out work
                cancelled.store(true, Ordering::Relaxed);
                return Err(e);
            }
        }

        Ok(())
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn every_item_is_processed_exactly_once() {
        let items: Vec<usize> = (0..100).collect();
        let mut results = Vec::new();

        for_each(
            &items,
            8,
            false,
            |item| item * 2,
            |_, value| {
                results.push(value);
                Ok(())
            },
        )
        .unwrap();

        results.sort();
        let expected: Vec<usize> = (0..100).map(|i| i * 2).collect();
        assert_eq!(results, expected);
    }

    /// items finish in reverse order of their input position, so anything but a deliberate
    /// re-ordering would show up here.
    #[test]
    fn ordered_emits_in_input_order() {
        let items: Vec<u64> = (0..10).collect();
        let mut indices = Vec::new();

        for_each(
            &items,
            10,
            true,
            |item| {
                std::thread::sleep(Duration::from_millis((10 - item) * 5));
                *item
            },
            |index, value| {
                indices.push((index, value));
                Ok(())
            },
        )
        .unwrap();

        let expected: Vec<(usize, u64)> = (0..10).map(|i| (i, i as u64)).collect();
        assert_eq!(indices, expected);
    }

    #[test]
    fn unordered_emits_everything_regardless_of_order() {
        let items: Vec<u64> = (0..10).collect();
        let mut values = Vec::new();

        for_each(
            &items,
            10,
            false,
            |item| {
                std::thread::sleep(Duration::from_millis((10 - item) * 5));
                *item
            },
            |_, value| {
                values.push(value);
                Ok(())
            },
        )
        .unwrap();

        values.sort();
        assert_eq!(values, (0..10).collect::<Vec<u64>>());
    }

    #[test]
    fn single_thread_keeps_input_order() {
        let items: Vec<usize> = (0..20).collect();
        let mut indices = Vec::new();

        for_each(
            &items,
            1,
            false,
            |item| *item,
            |index, _| {
                indices.push(index);
                Ok(())
            },
        )
        .unwrap();

        assert_eq!(indices, (0..20).collect::<Vec<usize>>());
    }

    #[test]
    fn a_failing_consumer_stops_the_run() {
        let items: Vec<usize> = (0..1000).collect();
        let mut seen = 0;

        let result = for_each(
            &items,
            4,
            false,
            |item| *item,
            |_, _| {
                seen += 1;
                anyhow::bail!("stop right there")
            },
        );

        assert!(result.is_err());
        assert_eq!(seen, 1);
    }

    #[test]
    fn empty_input_is_fine() {
        let items: Vec<usize> = Vec::new();

        for_each(&items, 8, false, |item| *item, |_, _| Ok(())).unwrap();
    }
}
