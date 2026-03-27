# Sandbox Provider Specs

> [!WARNING]
> The information in this directory was gathered by running standard Linux diagnostic commands (`uname`, `lscpu`, `cat /proc/*`, `dmesg`, `mount`, etc.) inside sandbox environments that we had legitimate access to as paying customers. No reverse engineering, decompilation, or unauthorized access was performed. WHOIS lookups use publicly available data. JWT payloads are standard base64-encoded JSON exposed in the sandbox's own environment variables, not encrypted secrets.
>
> That said, both [Anthropic's Terms of Service](https://www.anthropic.com/legal/terms) and [Namespace's Terms of Service](https://namespace.so/terms) include clauses prohibiting reverse engineering of their services. We believe runtime environment observation falls outside that scope, but **we are not lawyers**. If you are a representative of any provider documented here and believe this content should be modified or removed, please open an issue or contact us directly.
>
> Infrastructure details (cloud provider, hypervisor, networking architecture) are inferred from publicly observable signals, not confirmed by the providers. These inferences may be wrong.

## Providers

| Provider | Specs |
|---|---|
| [Namespace](namespace/) | S (8GB/4vCPU), M (16GB/8vCPU) |
| [Claude (Anthropic)](claude/) | Default (16GB/4vCPU) |

## Introspection script

The specs were generated using [`introspect.sh`](../introspect.sh), which outputs structured JSON. Run it inside any Linux sandbox:

```bash
bash introspect.sh > report.json
```
