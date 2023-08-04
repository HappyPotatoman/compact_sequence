# compact_sequence

[![CI](https://github.com/HappyPotatoman/compact_sequence/actions/workflows/healthcheck.yml/badge.svg)](https://github.com/HappyPotatoman/compact_sequence/actions/workflows/healthcheck.yml)

This is meant as a attempt to reduce mostly nucleotide/peptide sequence storage space.

# Changelog

0.2.1: Added support for undefined nucleotides 'N'

0.2.0: The new version features some major refactors and is in no way production ready nor is it backward compatible. The compress logic has changed completely. It is now based on an ASCII mapping for each possible basic triplet made up of the basic four nucleotides present in DNA (A, G, C, T). This way we can compress arbitrary sequences to a theoretical 1/3 of the file size. As a side note error handling was implemented partially, so the program is more stable and descriptive.

  - TODO: Test out actual sizes of files after compression.
  - TODO2: Implement support for RNA data (U instead of T). This could be achieved with using a flag.
  - For multiple sequences within the same file we could explore multithreaded processing.

0.1.0: As of now the program in implemented in a way that it groups reoccuring characters into the character and the amount of successive occurences e.g. AAA -> A3. This is an extremly efficient way to compress highly repetetive sequences. In some test runs when trying on real biological data e.g. Homo sapiens insulin gene and Drosophila melanogaster genome sequence data the compressed .txt files were ~85-90% of the original file size.
The current version also supports multithreaded processing of directories.

Currently supported formats to be compressed and unpacked:

  - .txt
  - .fasta, .fna, .ffn, .faa, .frn, .fa (coming soon)
