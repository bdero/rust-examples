#!/usr/bin/env python2
# Author: Brandon DeRosier <x@bdero.me>

"""Watch script for compiling Rust files."""

import logging
import os
import subprocess
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
            regexes=["^{0}\/\w*rust\w*\/[\w\.]+\/[\w]*\.rs$".format(source)],
            ignore_directories=True,
            case_sensitive=False
        )

        self._source = source
        self._destination = destination

    def do_conversion(self, event):
        """
        Compile the Rust programs.
        """
        source_path = event.src_path[:event.src_path.rfind("/")]
        destination_path = os.path.join(
            self._destination,
            source_path
        )
        source_file = os.path.join(source_path, "main.rs")
        destination_file = os.path.join(destination_path, "main")

        # Create build directory
        if not os.path.exists(destination_path):
            os.makedirs(destination_path)

        # Log the activity
        logging.info(
            "Compile: {} -> {}".format(
                source_file,
                destination_file
            )
        )

        commands = [
            # Command to compile a rust file
            (
                "COMPILER",
                "rustc -g -o {} {}".format(
                    destination_file,
                    source_file
                )
            ),
            # Command to execute the compiled binary
            (
                "PROGRAM",
                destination_file
            )
        ]

        for message, command in commands:
            try:
                # Run the command
                result = subprocess.check_output(
                    command,
                    stderr=subprocess.STDOUT,
                    shell=True
                )
            except subprocess.CalledProcessError, e:
                result = e.output
                logging.error(
                    "\033[91m\"{}\" returned exit status: {}\033[0m".format(
                        e.cmd,
                        e.returncode
                    )
                )

            # Log the results
            logging.info(
                "\033[91m{} OUTPUT:\033[0m\n{}".format(message, result)
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

    watch(".", "bin")
