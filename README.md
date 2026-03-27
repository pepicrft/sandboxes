# sandboxes

> [!NOTE]
> Last directory refresh: **March 27, 2026**. This repository is only a directory of sandbox options. If public information is missing, write `I don't know`.

## Provider Directory

| Provider | Service model | Notable capabilities | Pricing snapshot |
|---|---|---|---|
| [Daytona](https://www.daytona.io/) | Remote dev sandboxes / workspaces | sandbox lifecycle, exec, filesystem, snapshots, SDK + API | `I don't know` |
| [Namespace](https://namespace.so/) | Cloud development environments | persistent environments, SSH access, session-oriented workflows | `I don't know` |
| [Modal](https://modal.com/pricing) | Serverless containers and sandboxes | containerized exec, filesystem, autoscaling, GPU workloads | Starter: `$0` + usage, includes `$30/month` credits. Team: `$250/month` + usage, includes `$100/month` credits |
| [Fly.io](https://fly.io/) | Firecracker VMs and app machines | VM-style workloads, attached volumes, process execution | Usage-based infrastructure pricing |
| [Vercel Sandbox](https://vercel.com/docs/vercel-sandbox) | Ephemeral microVM sandboxes | command execution, file upload/download, automation-friendly SDK | `I don't know` |
| [Docker](https://www.docker.com/) | Self-hosted containers | local or remote containers, exec, archive-based filesystem workflows | Depends on your own infrastructure and Docker licensing |
| [Runloop](https://www.runloop.ai/) | Cloud browser / VM-like agent sandboxes | low-latency command execution, browser and environment automation | `I don't know` |
| [Cloudflare](https://developers.cloudflare.com/containers/) | Container workloads on Cloudflare infrastructure | containers plus surrounding Cloudflare primitives, API-driven orchestration | Depends on underlying Cloudflare products |
| [OpenComputer](https://docs.opencomputer.dev/sdks/typescript/overview) | API-managed sandboxes | commands, filesystem, PTY, template management | `I don't know` |
| [Blaxel](https://blaxel.ai/) | AI runtime / sandbox infrastructure | process execution and API-managed environments | `I don't know` |
| [OpenSandbox](https://opensandbox.com/) | Hosted sandbox environments | command execution and file workflows | `I don't know` |
| [Ona](https://ona.com/) | Cloud engineering environments | VM-backed workspaces, tasks, Gitpod migration path | Small `$0.12/hr`, Regular `$0.23/hr`, Large `$0.46/hr`, XL `$1.84/hr`, GPU `$1.95/hr` |
| [GitHub Codespaces](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-codespaces/about-billing-for-github-codespaces) | Hosted development VMs | SSH/terminal workflows, persistent dev environments, prebuilds | Metered by machine type, storage, and transfer |
| [Coder](https://coder.com/) | Self-hosted remote workspaces | templates, workspace lifecycle, SSH/IDE integrations | `I don't know` |
| [Devin](https://cognition.ai/) | Managed AI engineer environment | agent-driven sandbox/session workflows | `I don't know` |

## Capability Comparison

| Provider | Isolation model | Command execution | File operations | Persistence / volumes | Snapshot-like workflow |
|---|---|---|---|---|---|
| Daytona | VM-style sandbox | Yes | Yes | Yes | Yes |
| Namespace | Remote dev environment | Yes | Yes | Yes | Partial / provider-specific |
| Modal | Container sandbox | Yes | Yes | Yes | Partial / filesystem-oriented |
| Fly.io | Firecracker VM | Yes | Partial / volume-centric | Yes | Partial / suspend or image-based workflows |
| Vercel Sandbox | microVM | Yes | Yes | Partial | Partial |
| Docker | Container | Yes | Yes | Yes | Partial / image-based |
| Runloop | Managed sandbox / VM | Yes | Yes | Yes | Provider-specific |
| Cloudflare | Container platform | Yes | Yes | Partial | No clear generic snapshot primitive |
| OpenComputer | Managed sandbox | Yes | Yes | Yes | Provider-specific |
| Blaxel | Managed runtime | Yes | Partial | Partial | No clear generic snapshot primitive |
| OpenSandbox | Hosted sandbox | Yes | Yes | Partial | No clear generic snapshot primitive |
| Ona | VM workspace | Yes | Partial | Yes | Partial |
| GitHub Codespaces | VM workspace | Yes | Yes | Yes | Partial / prebuilds rather than snapshots |
| Coder | Workspace platform | Yes | Yes | Yes | Partial / template-oriented |
| Devin | Managed session sandbox | Indirect / agent-mediated | Limited | Partial | No clear generic snapshot primitive |

## Maintenance Notes

- Only include publicly available information.
- If a provider does not expose public pricing, write `I don't know`.

## Source Notes

Pricing and capability summaries above were refreshed from official provider docs and pricing pages on March 27, 2026.

## License

MIT
