# streamparse
A streaming, incremental data parser in Rust that processes arbitrarily large inputs in bounded memory. Parses newline-delimited and record-structured formats from any byte stream, emitting events as data arrives without buffering the whole input — with a zero-copy tokenizer, pluggable format grammars, and a push-based parser core.
