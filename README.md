# LogRust
A log file viewer written in Rust using [iced-rs](https://github.com/iced-rs/iced)

Currently, unusable. It only counts occurrences of [specific log levels](https://logging.apache.org/log4j/2.0/log4j-api/apidocs/org/apache/logging/log4j/Level.html) and displays them in a UI.
Nevertheless, it does so in an extremely fast manner (on my machine, it takes around 15 seconds to scan a ~4.5 Gb file with ~23.5 million lines)

As the name [implies](https://en.wikipedia.org/wiki/Toys_%22R%22_Us), this is pretty much a toy project. I always wanted to get into Rust while also trying some kind of UI framework.
And I happened to need some library that could open and filter large logs by their log level so here we are

