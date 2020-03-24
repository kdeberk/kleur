
Sometimes certain CLI programs do not offer colored output, or sometimes you want to enhance this.

```
alias go=`kleur go`
```

`~/.config/kleur/config.yml`:
```
---
- command: go test
  rules:
    - regex: PASS
      color: green
    - regex: FAIL
      color: red
    - regex: \d{4}/\d{2}/\d{2}.*
      color: dim
```


