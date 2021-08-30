# Meu precioso

Exercício - Anel Multithread

- Usando uma linguagem de alto-nível como C/C++/Java, escrever um programa que crie 30 threads e faça com que uma mensagem circule entre os mesmos.

- A mensagem é uma string aleatória de pelo menos 80 caracteres.

- A cada vez que um thread recebe a mensagem ele a imprime, modifica o primeiro caractere minúsculo para maiúsculo, caso exista, dorme por 1 segundo, e repassa a mensagem.

- Quando todos os caracteres forem maiúsculos, o processo repassa a mensagem e então termina.

- Antes de terminar, o processo deve imprimir a mensagem resultante.


Para resolver o problema foi utilizado rust e seu
Toolkit de desenvolvimento então para executar
aplicação basta:
```shell
cargo run
```

__exemplo de saída__

```shell
message: FQfYOMedopzQzbdjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT
thread: 1 message: FQfYOMedopzQzbdjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 2 message: FQFYOMedopzQzbdjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 3 message: FQFYOMEdopzQzbdjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 4 message: FQFYOMEDopzQzbdjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 5 message: FQFYOMEDOpzQzbdjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 6 message: FQFYOMEDOPzQzbdjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 7 message: FQFYOMEDOPZQzbdjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 8 message: FQFYOMEDOPZQZbdjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 9 message: FQFYOMEDOPZQZBdjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 10 message: FQFYOMEDOPZQZBDjrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 11 message: FQFYOMEDOPZQZBDJrhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 12 message: FQFYOMEDOPZQZBDJRhFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 13 message: FQFYOMEDOPZQZBDJRHFQKBpWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 14 message: FQFYOMEDOPZQZBDJRHFQKBPWQEeRguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 15 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERguBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 16 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGuBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 17 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUqKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 18 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKqahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 19 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQahqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 20 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAhqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 21 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHqCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 22 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCklrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 23 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKlrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 24 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLrTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 25 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGmsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 26 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMsjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 27 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSjeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 28 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJeJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 29 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMyyJsVGBThTWYBQebOhhAUOAT 
thread: 30 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYyJsVGBThTWYBQebOhhAUOAT 
thread: 1 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJsVGBThTWYBQebOhhAUOAT 
thread: 2 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBThTWYBQebOhhAUOAT 
thread: 3 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQebOhhAUOAT 
thread: 4 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEbOhhAUOAT 
thread: 5 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOhhAUOAT 
thread: 6 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHhAUOAT 
thread: 7 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 8 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 9 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 10 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 11 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 12 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 13 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 14 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 15 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 16 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 17 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 18 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 19 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 20 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 21 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 22 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 23 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 24 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 25 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 26 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 27 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 28 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 29 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 30 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 1 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 2 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
thread: 3 message: FQFYOMEDOPZQZBDJRHFQKBPWQEERGUBUBCAUQKQAHQCKLRTKKGMSJEJMYYJSVGBTHTWYBQEBOHHAUOAT 
```

# Explicações em vídeo

[![Explicando o meu anel](http://img.youtube.com/vi/DQWoig4bJrw/0.jpg)](https://youtu.be/DQWoig4bJrw "Explicação em vídeo")

não sei contar até 30, mil perdões :(