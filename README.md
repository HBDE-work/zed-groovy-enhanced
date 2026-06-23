# Groovy Zed Extension

> [!NOTE]
> This fork intends to enhance the groovy support in zed.
> Thanks to https://github.com/valentinegb for the initial implementation!

---

Zed Groovy support.

<img width="1092" alt="Screenshot 2024-05-16 at 10 07 07 PM" src="https://github.com/valentinegb/zed-groovy/assets/35977727/029d56f4-5852-4692-a98f-d42b1388f3e4">

## Enhancements

### support specific java version for groovy LSP

Status: **Implemented**

#### Config Example
```jsonc
"lsp" : {
    "groovy-enhanced": {
        "settings": {
            "javaHome": "/usr/lib/jvm/java-11-openjdk-amd64", // configure matching java for the system groovy version
        },
    },
}
```

### fix LSP deserialization errors

Status: **TODO**
