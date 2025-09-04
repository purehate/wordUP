# WORD UP
# Content-Aware Wordlist Generator
# Extracts words from live subdomains and expands via Markov chains

import requests
import re
import sys
import time
import random
import string
import dns.resolver
from bs4 import BeautifulSoup
from urllib.parse import urljoin, urlparse
from collections import defaultdict, Counter

# Settings
COMMON_SUBS = ["www", "mail", "webmail", "vpn", "remote", "portal", "admin", "login", "app", "cloud", "dev"]
USER_AGENT = "Mozilla/5.0 (TrustedSec Word Up)"
HEADERS = {"User-Agent": USER_AGENT}
MIN_WORD_LEN = 4
CRAWL_DEPTH = 1
EXPANSION_MULTIPLIER = 30

def get_subdomains_crtsh(domain):
    print("[+] Pulling subdomains from crt.sh")
    url = f"https://crt.sh/?q=%25.{domain}&output=json"
    try:
        r = requests.get(url, timeout=10)
        data = r.json()
        subs = set()
        for entry in data:
            name = entry['name_value']
            for sub in name.split("\n"):
                if domain in sub:
                    subs.add(sub.strip())
        return sorted(subs)
    except Exception as e:
        print(f"[!] Error fetching crt.sh: {e}")
        return []

def brute_subdomains(domain):
    print("[+] Brute-forcing common subdomains")
    found = []
    for sub in COMMON_SUBS:
        fqdn = f"{sub}.{domain}"
        try:
            dns.resolver.resolve(fqdn, 'A')
            found.append(fqdn)
        except:
            continue
    return found

def check_http_live(subdomains):
    print("[+] Checking which subdomains are live")
    live = []
    for sub in subdomains:
        for scheme in ["https://", "http://"]:
            try:
                url = scheme + sub
                r = requests.get(url, headers=HEADERS, timeout=5)
                if r.status_code < 400:
                    live.append(url)
                    break
            except:
                continue
    return live

def extract_words_from_url(url):
    print(f"    [.] Scraping {url}")
    words = set()
    try:
        r = requests.get(url, headers=HEADERS, timeout=10)
        soup = BeautifulSoup(r.text, "html.parser")
        texts = soup.stripped_strings
        for line in texts:
            for word in re.findall(r"\b[a-zA-Z]{%d,}\b" % MIN_WORD_LEN, line):
                words.add(word.lower())
    except:
        pass
    return words

def build_markov_chain(words, order=2):
    print("[+] Building Markov model")
    model = defaultdict(Counter)
    for word in words:
        padded = "~" * order + word
        for i in range(len(word)):
            prefix = padded[i:i+order]
            next_char = word[i]
            model[prefix][next_char] += 1
    return model

def generate_words_from_markov(model, count=1000, order=2):
    print(f"[+] Generating {count} expanded words from model")
    results = set()
    while len(results) < count:
        prefix = "~" * order
        word = ""
        for _ in range(20):  # max length
            choices = model[prefix]
            if not choices:
                break
            next_char = random.choices(list(choices), weights=choices.values())[0]
            word += next_char
            prefix = prefix[1:] + next_char
        if len(word) >= MIN_WORD_LEN:
            results.add(word)
    return results

def main(domain):
    subdomains = set(get_subdomains_crtsh(domain)) | set(brute_subdomains(domain))
    live_hosts = check_http_live(subdomains)

    all_words = set()
    for url in live_hosts:
        words = extract_words_from_url(url)
        all_words.update(words)

    print(f"[+] Total unique words extracted: {len(all_words)}")

    # Save raw wordlist
    with open("wordlist_raw.txt", "w") as f:
        for w in sorted(all_words):
            f.write(w + "\n")

    # Expand with Markov
    model = build_markov_chain(all_words)
    expanded = generate_words_from_markov(model, count=len(all_words)*EXPANSION_MULTIPLIER)

    with open("wordlist_expanded.txt", "w") as f:
        for w in sorted(expanded):
            f.write(w + "\n")

    print("[+] Wordlist generated: wordlist_expanded.txt")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <company_domain>")
        sys.exit(1)
    main(sys.argv[1])

