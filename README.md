# compact_sequence

[![CI](https://github.com/HappyPotatoman/compact_sequence/actions/workflows/healthcheck.yml/badge.svg)](https://github.com/HappyPotatoman/compact_sequence/actions/workflows/healthcheck.yml)

# Efficient storage for nucleotide sequence data

usage cargo run -- <input_file or directory> <-u for unpacking compressed data>

# Changelog

0.3.2: Add directory unpacking. Define output folders with -o or --output. Refactored the codebase to enable easier addition of new file types.

0.3.1: Add support for RNA sequential data. Add better command line control with StructOpt.

0.3.0: "NNN" is encoded with a single char to optimize space usage. Multithread processing of single files. Check performance benchmarks for details.

0.2.1: Added support for undefined nucleotides 'N'.

0.2.0: The new version features some major refactors and is in no way production ready nor is it backward compatible. The compress logic has changed completely. It is now based on an ASCII mapping for each possible basic triplet made up of the basic four nucleotides present in DNA (A, G, C, T). This way we can compress arbitrary sequences to a theoretical 1/3 of the file size. As a side note error handling was implemented partially, so the program is more stable and descriptive.

0.1.0: As of now the program in implemented in a way that it groups reoccuring characters into the character and the amount of successive occurences e.g. AAA -> A3. This is an extremly efficient way to compress highly repetetive sequences. In some test runs when trying on real biological data e.g. Homo sapiens insulin gene and Drosophila melanogaster genome sequence data the compressed .txt files were ~85-90% of the original file size.
The current version also supports multithreaded processing of directories.

Currently supported formats to be compressed and unpacked:

  - .txt

# Performance benchmarks

v0.2.1 and earlier the compress and decompress functionalities were running as single threaded sequential processes. There was a multithreaded option for directories, but that only meant each file was given a single thread. We have tried the performance on the following benchmarks:

| Source       | Original (in kB)   | Compressed (in kB) | Compress Time (in s) | Decompress Time (in s) | Version         |
|--------------|--------------------|--------------------|----------------------|------------------------|-----------------|
| Drosophila   | 47,131             |16,095              | 144                  | 124                    |0.2.1            |
| Drosophila   | 47,131             |16,095              | 2                    | 1.9                    |0.3.0            |

# Feature backlog

 - Add other sequential data formats such as FASTA,
 - Add ignore non supported files flag when passing in directories,
 - Add debug funtionality for performance testing and timing function execution times,
 - Improve error handling,
 - Check for bottlenecks using a profiler,
 - GUI for easier backup management,