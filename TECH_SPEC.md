A rust based CLI program called llm-cli (alias llm). The goal is to get quick answers from the AI inside terminal, e.g:

```
> llm -s "What command shall I use to see all files and folders including hidden ones"
ls -la
```

---

The program config should be in `.config/llm-cli/config.toml`. Config options
- Base url (OpenAI compatible Chat Completions API)
- API key
- model id
- system instuction (the default should be set during installation: "You are an AI CLI assistant. Your answers should be short and concise. If the user asks for command, output just command without any comments." 

---

Two modes:
- Interactive (default): a dialog between user and program. All previous messages should be included in API request 
```
> llm -s "What command shall I use to see all files"
ls
> "And how include hidden ones"
ls -la
```

- Non interactive (with -s or --single argument): A single request
```
> llm -s "What command shall I use to see all files and folders including hidden ones"
ls -la
```

---

Provider errors (404, 403, etc.) should be displayed via notify-send.
