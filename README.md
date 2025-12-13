# ArcticOJ

Just another ArcticOJ rewrite, but now to Rust.
> With the same philosophy as Go-based `ArcticOJ`, (not-so) lightweight & (not-so) portable.

> As with Go-based `ArcticOJ`, this project stands as a playground for me to experiment new stuff so its stability & ETA remains questionable. 

## TODOs

- [ ] Sandboxing
- [ ] Bridge server
- [ ] RBAC
- [ ] Problems
- [ ] Contests
- [ ] Teams
- [ ] Interactive problems

and a lot more...

## Crates

| Name    | Desc                                                                                                                                                                                      |
|---------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `blizzard` | API server                                                                                                                                                                                |
| `igloo` | Judge server (with sandbox)                                                                                                                                                               |
| `polar` | Bridge interface between `blizzard` & `igloo`<br/>(integrated into `blizzard`)                                                                                                            |
| `igloo` | Zygote program to exec code inside container<br/>(to reduce container initialization overhead by using a preforked one)<br/>(Not sure if its needed but its kinda complicated to impl...) ||

[//]: # (## Running as unprivileged user &#40;rootless&#41; &#40;WIP&#41;)

[//]: # (```shell)

[//]: # (sudo setcap )

[//]: # (sudo vi /etc/cgconfig.conf)

[//]: # (```)

[//]: # (```)

[//]: # (group igloo.slice {)

[//]: # (    perm {)

[//]: # (        task {)

[//]: # (                uid = 1000;)

[//]: # (        })

[//]: # (        admin {)

[//]: # (                uid = 1000;)

[//]: # (        })

[//]: # (    })

[//]: # (	cpuset {})

[//]: # (	cpu {})

[//]: # (	io {})

[//]: # (	memory {})

[//]: # (	pids {})

[//]: # (})

[//]: # (```)

[//]: # (```shell)

[//]: # (sudo systemctl start cgconfig.service)

[//]: # (```)