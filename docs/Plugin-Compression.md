#### Compression Plugin (`compression.rs`)

```rust
pub trait CompressionPlugin: Plugin {
    fn compress(
        &self,
        input_files: &[PathBuf],
        output_file: &PathBuf,
        options: &CompressionOptions,
    ) -> Result<(), PluginError>;

    fn decompress(
        &self,
        input_file: &PathBuf,
        output_dir: &PathBuf,
        overwrite: bool,
    ) -> Result<(), PluginError>;
}
```
