# rinnova

[![BSD 3 Clause License](https://img.shields.io/github/license/nigeleke/rinnova?style=plastic)](https://github.com/nigeleke/rinnova/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/rinnova/ci.yml?style=plastic)](https://github.com/nigeleke/rinnova/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/rinnova?style=plastic)](https://codecov.io/gh/nigeleke/rinnova)
![Version](https://img.shields.io/github/v/tag/nigeleke/rinnova?style=plastic)

  [Site](https://nigeleke.github.io/rinnova) \| [GitHub](https://github.com/nigeleke/rinnova) \| [App](https://nigeleke.github.io/rinnova/app/)

`rinnova` is a simple, private application for managing prescriptions and medicine supplies; `rinnova` is Italian for renewal.

## Background

`rinnova` helps you keep track of your medicines, prescriptions, repeat supplies and dispensing history so you always know what is available, what needs renewing, and what actions to take next.

Unlike a calendar or reminder app, `rinnova` understands how prescriptions work. It records which medicines appear on each prescription, how many repeat supplies were authorised, and each time a medicine is dispensed. The app will manage and display:

  - Which prescriptions are current, due to expire, expired or exhausted.
  - How many supplies (scripts, not dosages) remain for each medicine.
  - Whether a medicine is still covered by a valid prescription.
  - When a new prescription should be requested.
  - A complete history of prescriptions and supply.

## Features

  - Record medicines once and reuse them across multiple prescriptions.
  - Track prescriptions, including issue and expiry dates.
  - Record dispensings from pharmacies.
  - Automatically calculate remaining supplies.
  - View medicine and prescription health at a glance.
  - Receive reminders before prescriptions expire or repeat supplies run out.
  - Search and browse your prescription history.

## Privacy

All information is stored locally on your device. `rinnova` does not require an online account and does not upload your personal information to cloud services.

## Not Medical Advice

`rinnova` is a personal organiser. It does not provide medical advice and is not a substitute for guidance from a doctor or pharmacist. You remain responsible for checking prescription validity, medicine availability and dispensing information.

## Development

```bash
dx build
cargo test
dx serve
```

- Open the browser to http://localhost:8080/
