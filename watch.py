#!/usr/bin/env python2
# Author: Brandon DeRosier <x@bdero.me>

"""Watch script for compiling Rust files."""

import logging
import time

from watchdog.observers import Observer
from watchdog.events import RegexMatchingEventHandler


class RustConversionEventHandler(RegexMatchingEventHandler):
    """
    Match rust files in the source directory and compile them in the
    destination directory.
    """
    def __init__(self, source, destination):
        super(RustConversionEventHandler, self).__init__(
            regexes=["^{0}\/\d+\_[\w]+\/[\w]*\.rs$".format(source)],
            ignore_directories=True,
            case_sensitive=False
        )

        self._source = source
        self._destination = destination

    def do_conversion(self, event):
        """
        Compile the Rust programs.
        """
        file_path = event.src_path[len(self._source) + 1:-3]

        # Compile the rust file
        logging.info(
            "Compile: {} -> {}".format(
                event.src_path,
                "{}/{}".format(self._destination, file_path)
            )
        )

    def on_modified(self, event):
        """
        Process modified markdown files.
        """
        self.do_conversion(event)

    def on_created(self, event):
        """
        Process newly created markdown files.
        """
        self.do_conversion(event)


def watch(source, destination):
    """
    Watch the source directory for changes until a KeyboardInterrupt is
    encountered.
    """
    observer = Observer()
    event_handler = RustConversionEventHandler(source, destination)
    observer.schedule(event_handler, source, recursive=True)

    logging.info("Starting watcher..")

    try:
        observer.start()
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        logging.info("Keyboard interrupt received; stopping watcher..")
        observer.stop()

    observer.join()


if __name__ == "__main__":
    logging.basicConfig(
        level=logging.INFO,
        format='[%(asctime)s] %(levelname)s: %(message)s',
        datefmt='%Y-%m-%d %H:%M:%S'
    )

    watch("rust_by_example", "bin")
