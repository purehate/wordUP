# WORD UP  
*Content-Aware Wordlist Generator*  
_by TrustedSec_

```

## What It Does

**Word Up** is a domain-targeted wordlist generator that:

- Pulls subdomains for a company (via `crt.sh` + DNS brute force)
- Visits each reachable web host and extracts **all usable words**
- De-duplicates and trains a **Markov chain model**
- Expands the list 20–30× with generated words that feel "human"
- Outputs a rock-solid wordlist for password testing, spraying, or rule-building

---

## Use Cases

- 🔍 Password spraying with domain-specific terms  
- 🔓 Building realistic internal test accounts  
- 🧠 Training your own Markov or PCFG models  
- 🔬 Feeding red team/offensive automation  
- 🧱 Fueling detection engineering for weird login attempts

---

## How to Use

```bash
# Install required packages
pip install requests beautifulsoup4 dnspython

# Run the tool
python3 word_up.py <target-company.com>
```

It will:
- Extract all words from live sites
- Generate `wordlist_raw.txt` and `wordlist_expanded.txt` in the current directory

---

## 🧠 How It Works

1. 🔎 **Subdomain Discovery**
   - Queries `crt.sh` for cert transparency
   - Tries common subdomains like `vpn`, `portal`, `mail`

2. 🌐 **Crawling**
   - Hits live subdomains with HTTP/HTTPS
   - Scrapes visible text via `BeautifulSoup`

3. 🧹 **Normalization**
   - Filters out short/noisy tokens
   - Lowercases and deduplicates everything

4. 📈 **Markov Expansion**
   - Builds trigram-based Markov model
   - Outputs thousands of “realistic” looking guesses

---

## Sample Output

```text
# wordlist_raw.txt
trustedsec
services
cybersecurity
incident
assessment

# wordlist_expanded.txt
trustaware
servicingly
cyberanics
incidentguard
assesswise
...
```

---

## Coming Soon (or Fork It Yourself)

- ✅ Leetspeak + rule-style mutations
- ✅ Add names/emails from `about` pages
- ✅ PCFG grammar training option
- ✅ Hybrid generator mode (like OMEN/John)

---

## Disclaimer

**Word Up** is for ethical testing, research, and detection engineering use only.  
Don’t be a Gibson script kiddie.

---

*“It’s all about who controls the information.”* – Cosmo, *Sneakers*  

