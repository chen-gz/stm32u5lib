# Gemini Rules

- Do not include `TAG=` and `CONV=` tags in commit descriptions.
Interrupts do not modify registers (they can only mask the interrupt resgister), so race conditions will not occur.
