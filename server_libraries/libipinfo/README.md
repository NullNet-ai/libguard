## libipinfo

**IP information** library for Nullnet.

It allows specifying one or more IP information providers and querying them for information about an IP address.<br>
Providers can be of two types:
- **API providers**, queried by making an HTTP request to a specific URL
- **MMDB providers**: queried by looking up the IP address in a database

This library also exposes a way to configure the JSON schema for the responses of the API providers,
and permits to download and periodically refresh MMDB databases.
