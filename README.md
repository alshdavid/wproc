# wproc

Will watch `./file.txt` for changes and run `cat ./file.txt` when changes are detected

```bash
wproc -w ./file.txt -- cat ./file.txt 
```