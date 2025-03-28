use std::sync::Arc;
use std::time::Duration;
use tokio::io::{
    self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadHalf, WriteHalf, split,
};
use tokio::sync::Notify;
use tokio::time::{self, Instant};

/// Copies data bidirectionally between two streams (`stream1` and `stream2`) with an idle timeout.
/// If no data is transferred for the specified `timeout` duration, the operation will stop.
///
/// # Arguments
/// * `stream1` - The first stream implementing `AsyncRead` and `AsyncWrite`.
/// * `stream2` - The second stream implementing `AsyncRead` and `AsyncWrite`.
/// * `timeout` - The maximum duration of inactivity before stopping the operation.
///
/// # Behavior
/// This function runs until either:
/// 1. Both streams are closed (EOF is reached).
/// 2. The idle timeout is reached (no data is transferred for the specified duration).
pub async fn copy_bidirectional_with_timeout<T, U>(stream1: T, stream2: U, timeout: Duration)
where
    T: AsyncRead + AsyncWrite + Send + 'static,
    U: AsyncRead + AsyncWrite + Send + 'static,
{
    let notify = Arc::new(Notify::new());

    tokio::select! {
        _ = copy_streams_bidirectional_with_notify(stream1, stream2, notify.clone()) => {},
        _ = wait_for_idle_timeout(notify, timeout) => {}
    }
}

/// Waits for an idle timeout to occur, resetting the timer whenever notified.
///
/// # Arguments
/// * `notify` - An `Arc<Notify>` used to reset the idle timer.
/// * `timeout` - The maximum duration of inactivity before breaking the loop.
///
/// # Behavior
/// This function resets the idle timer whenever notified (i.e., when data is transferred).
/// If the timeout elapses without any notifications, the loop breaks.
async fn wait_for_idle_timeout(notify: Arc<Notify>, timeout: Duration) {
    let mut timestamp = Instant::now();

    loop {
        tokio::select! {
            _ = notify.notified() => {
                timestamp = Instant::now();
            },
            _ = time::sleep(timeout) => {
                if timestamp.elapsed() >= timeout {
                    break;
                }
            }
        }
    }
}

/// Copies data bidirectionally between two streams, notifying when data is transferred.
///
/// # Arguments
/// * `stream1` - The first stream implementing `AsyncRead` and `AsyncWrite`.
/// * `stream2` - The second stream implementing `AsyncRead` and `AsyncWrite`.
/// * `notify` - An `Arc<Notify>` used to signal when data is transferred.
///
/// # Behavior
/// This function splits the streams into readers and writers, then spawns two tasks to copy data
/// in both directions. It notifies the `notify` mechanism whenever data is transferred.
async fn copy_streams_bidirectional_with_notify<T, U>(stream1: T, stream2: U, notify: Arc<Notify>)
where
    T: AsyncRead + AsyncWrite + Send + 'static,
    U: AsyncRead + AsyncWrite + Send + 'static,
{
    let (reader1, writer1) = split(stream1);
    let (reader2, writer2) = split(stream2);

    let _ = tokio::join!(
        tokio::spawn(copy_direction_with_notify(reader1, writer2, notify.clone())),
        tokio::spawn(copy_direction_with_notify(reader2, writer1, notify.clone())),
    );
}

/// Copies data from a reader to a writer, notifying when data is transferred.
///
/// # Arguments
/// * `reader` - The reader half of a stream implementing `AsyncRead`.
/// * `writer` - The writer half of a stream implementing `AsyncWrite`.
/// * `notify` - An `Arc<Notify>` used to signal when data is transferred.
///
/// # Returns
/// Returns `Ok(())` if the operation completes successfully, or an `io::Error` if an error occurs.
///
/// # Behavior
/// This function reads data from the `reader` into a buffer and writes it to the `writer`.
/// It notifies the `notify` mechanism whenever data is transferred.
/// If the reader reaches EOF (returns `Ok(0)`), the function returns successfully.
/// If an error occurs during reading or writing, the function returns the error.
async fn copy_direction_with_notify<T, U>(
    mut reader: ReadHalf<T>,
    mut writer: WriteHalf<U>,
    notify: Arc<Notify>,
) -> io::Result<()>
where
    T: AsyncRead,
    U: AsyncWrite,
{
    let mut buffer = [0; 8192];
    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => return Ok(()),
            Ok(n) => {
                notify.notify_one();
                match writer.write_all(&buffer[..n]).await {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                }
            }
            Err(err) => return Err(err),
        }
    }
}
