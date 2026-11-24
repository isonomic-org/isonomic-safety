# Security policy

## Reporting a vulnerability

Report vulnerabilities privately through GitHub: open the **Security** tab of the affected repository and choose **Report a vulnerability**. Do not open a public issue, pull request or discussion for a suspected vulnerability.

Please include:

- the repository and the commit or release you tested;
- what an attacker can do, and from which position (a web page or file the agent reads, a third-party View, a Cell, a local process);
- steps to reproduce, and a proof of concept if you have one.

## What happens next

We acknowledge the report, confirm or rule out the issue, and agree a disclosure date with you. Fixed issues are published as security advisories in the affected repository, with credit to the reporter unless you ask otherwise.

## Scope

In scope: the controls described in [docs/threat-model.md](docs/threat-model.md), wherever they are implemented, and the evaluation code in this repository.

Out of scope: attacks that require an already compromised user device (see the threat model), and findings in third-party models themselves. A model that follows an injected instruction is expected; a control that lets the resulting action through is a vulnerability.
