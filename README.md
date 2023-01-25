# slr1aux
Ferramenta para gerar automaticamente o automato SLR(1).
## Execução
```
cargo run -- caminho/para/o/arquivo/da/gramatica.txt caminho/para/o/arquivo/do/follow.txt
```
## Formato da gramática
```
S' -> T
T -> F
T -> T * F
F -> id
F -> ( T )
```
OBS: Quando um follow não possui terminais, então deve-se adicionar um espaço após o ```:```.
## Exemplo de tabela gerada
| Estado | * | id | ( | ) | $ | T | F |
|---|---|---|---|---|---|---|---|
| I0 | ```erro``` | ```E5``` | ```E6``` | ```erro``` | ```erro``` | ```1``` | ```4``` | 
| I1 | ```E2``` | ```erro``` | ```erro``` | ```erro``` | ```ACEITAR``` |   |   | 
| I2 | ```erro``` | ```E5``` | ```E6``` | ```erro``` | ```erro``` |   | ```3``` | 
| I3 | ```R2``` | ```R2``` | ```R2``` | ```R2``` | ```R2``` |   |   | 
| I4 | ```R1``` | ```R1``` | ```R1``` | ```R1``` | ```R1``` |   |   | 
| I5 | ```R3``` | ```R3``` | ```R3``` | ```R3``` | ```R3``` |   |   | 
| I6 | ```erro``` | ```E5``` | ```E6``` | ```erro``` | ```erro``` | ```7``` | ```4``` | 
| I7 | ```E2``` | ```erro``` | ```erro``` | ```E8``` | ```erro``` |   |   | 
| I8 | ```R4``` | ```R4``` | ```R4``` | ```R4``` | ```R4``` |   |   | 