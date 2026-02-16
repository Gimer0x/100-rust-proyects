# d96_toml_pdf – Resume builder (TOML → HTML → PDF)

Build the project and run:

```bash
cargo run
```

The tool reads `resume.toml`, generates `resume.html`, and optionally `resume.pdf` if **wkhtmltopdf** is installed.

---

## Installing wkhtmltopdf on macOS

Homebrew no longer provides wkhtmltopdf (`brew install wkhtmltopdf` fails). Use one of these options:

### Option 1: Official installer (Intel Macs, macOS 10.7+)

1. Download the macOS package from GitHub:
   - **https://github.com/wkhtmltopdf/packaging/releases/download/0.12.6-2/wkhtmltox-0.12.6-2.macos-cocoa.pkg**
2. Open the downloaded `.pkg` and run the installer.
3. Ensure the install location is on your PATH (often `/usr/local/bin`). If needed, add to `~/.zshrc`:
   ```bash
   export PATH="/usr/local/bin:$PATH"
   ```

### Option 2: MacPorts (if you use MacPorts)

```bash
sudo port install wkhtmltopdf
```

### Option 3: Run without PDF

If you don’t install wkhtmltopdf, the program still runs and produces `resume.html` only. You’ll see:

```
wkhtmltopdf not found or failed. HTML only.
```

You can open `resume.html` in a browser and use “Print → Save as PDF” to get a PDF.
