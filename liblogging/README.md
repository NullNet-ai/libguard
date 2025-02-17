## liblogging

Logging and error handling library for Nullnet.

It currently logs to the **console** and to **syslog**.

You can configure the log level by setting the `LOG_LEVEL` environment variable. The possible log levels are:
- `OFF`
- `ERROR`
- `WARN`
- `INFO`
- `DEBUG`
- `TRACE`

If you don't set the `LOG_LEVEL` environment variable, `TRACE` will be used by default (the most verbose level).

Use this library simply by calling the `Logger::init` method, and then invoking the `log` macros.

This library also exports an `ErrorHandler` trait that you can use to log and unify error handling in your program.
