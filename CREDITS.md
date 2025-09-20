# Credits & Attribution

This project draws inspiration and techniques from several excellent open-source projects in the security community. We are grateful for their contributions and the techniques that inspired this tool.

## Primary Inspiration

### CeWL (Custom Word List generator)
- **Author**: Digininja
- **Repository**: https://github.com/digininja/CeWL
- **License**: GPL-3.0
- **Contribution**: Original concept for web-based wordlist generation, HTML parsing techniques, and email extraction methods

### hashcat-utils
- **Author**: Hashcat Team
- **Repository**: https://github.com/hashcat/hashcat-utils
- **License**: MIT
- **Contribution**: Advanced password cracking utilities that inspired our word processing techniques:
  - `expander.bin` - Word expansion techniques
  - `cutb.bin` - Cut-based word processing
  - `combinator.bin` - Word combination methods
  - `rli2.bin` - Rule generation techniques
  - `maskgen.bin` - Mask generation patterns

### evilmog/hashcat-scripts
- **Author**: EvilMog
- **Repository**: https://github.com/evilmog/hashcat-scripts
- **License**: MIT
- **Contribution**: Random hashcat scripts and advanced attack methodologies:
  - `fingercut.sh` - Dynamic expander and fingerprint attacks
  - Iterative refinement techniques
  - Hybrid attack patterns
  - Comprehensive attack methodology

### PACK (Password Analysis and Cracking Kit)
- **Author**: Peter Kacherginsky (iphelix)
- **Repository**: https://github.com/iphelix/pack
- **License**: BSD-3-Clause
- **Contribution**: Comprehensive password analysis toolkit:
  - `statsgen.py` - Password statistics and pattern analysis
  - `policygen.py` - Password policy analysis
  - `rulegen.py` - Advanced rule generation with edit distance
  - `maskgen.py` - Pattern-based mask generation

## Techniques Integrated

| Technique | Inspired By | Description |
|-----------|-------------|-------------|
| **Expander Technique** | hashcat-utils/expander.bin | Generates word variations with prefixes, suffixes, and special characters |
| **Cutb Technique** | hashcat-utils/cutb.bin | Cuts words at different positions (beginning, end, both ends) |
| **Combinator Technique** | hashcat-utils/combinator.bin | Combines words from two lists with separators |
| **RLI2 Technique** | hashcat-utils/rli2.bin | Generates hashcat rules based on word patterns |
| **Maskgen Technique** | hashcat-utils/maskgen.bin | Generates masks based on word length patterns |
| **Prince Technique** | princeprocessor | Generates word combinations and mutations |
| **Iterative Refinement** | EvilMog's methodology | Repeatedly processes wordlists to find new variations |
| **PACK StatsGen** | PACK/statsgen.py | Analyzes password statistics and patterns |
| **PACK PolicyGen** | PACK/policygen.py | Analyzes password policies and requirements |
| **PACK RuleGen** | PACK/rulegen.py | Generates advanced rules with edit distance |
| **PACK MaskGen** | PACK/maskgen.py | Generates pattern-based masks |
| **Web Scraping** | CeWL | Extracts words from websites using HTML parsing |
| **Email Extraction** | CeWL | Extracts email addresses from mailto links and content |

## Special Thanks

- **EvilMog** - For the comprehensive hashcat methodology and advanced attack patterns
- **Hashcat Team** - For the excellent utility tools and documentation
- **Peter Kacherginsky (iphelix)** - For the PACK toolkit and password analysis techniques
- **Digininja** - For the original CeWL concept and implementation
- **Rust Community** - For the amazing ecosystem and performance optimizations
- **Security Community** - For the collaborative spirit and knowledge sharing

## License Compatibility

This project is released under the MIT License, which is compatible with the licenses of the projects that inspired it:
- CeWL: GPL-3.0 (concepts and techniques, not code)
- hashcat-utils: MIT (concepts and techniques, not code)
- evilmog/hashcat-scripts: MIT (concepts and techniques, not code)

## Disclaimer

This tool is for educational and authorized security testing purposes only. Users are responsible for ensuring they have proper authorization before using this tool on any systems or networks.

---

*"Standing on the shoulders of giants" - This project builds upon the excellent work of the security community and aims to contribute back to the ecosystem.*
