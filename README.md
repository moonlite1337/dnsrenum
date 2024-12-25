## Intro

A repo is a work in progress.
This is an attempt to rewrite perl script into a binary:
https://github.com/fwaeytens/dnsenum

Note: kali uses different (more up to date) version of dnsenum script.

Features to add:

- [x] basic DNS lookup
- [x] help string
- [x] add dns list
- [x] zone transfer
- [x] google first page scrape (optional)
- [=] brute force using dns presets
- [ ] whois C class domain network ranges
- [ ] reverse lookup on c class network ranges
- [ ] Write to domain_ips.txt file non-contiguous ip-blocks results
- [ ] configure release mgmt
- [ ] kali package
- [ ] find good to add features from subfinder and similar

## notes

recommended DNS list:
https://github.com/danielmiessler/SecLists/tree/master/Discovery/DNS

current data/example.txt is copied from:
https://github.com/SparrowOchon/dnsenum2/blob/master/dns.txt
