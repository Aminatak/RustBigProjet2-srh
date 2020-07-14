## Questions / Réponses

### 1.2 Questions : Rappels de Rust, généralités 

1. En Rust à quoi servent les références ?

Les références servent à accéder directement aux variables en mode lecture et/ou écriture sans créer une copie dans la mémoire .

2. Citez en Rust les grandes façons de déclarer ses propres types.

* Structure
* TYPE
* Union (pareil que Struct)

3. Rust est compilé nativement (assembleur sous forme de code machine) ou compte sur une machine virtuelle pour l’exécuter ?

Rust est compilé par une machine virtuelle (LLVM:Low Level Virtual Machine)

4. Imaginons qu'on a un système avec un processeur 8bits, quelle est la valeur maximale adressable ? Écrire la solution en notation hexadécimale et décimale.*

“Les processeurs 8 bits utilisent normalement un bus de données 8 bits et un bus d'adresses 16 bits"  donc :

 - en décimale : 2^16 = **65536 octets**
 - en hexadécimale : **#FFFF**

5. Donnez votre définition d'un processus. Citez vos sources ! 

Un processus est un programme en cours d'exécution. Il en existe 2 types:
    - Les processus systèmes
    - Les processus utilisateurs 
Source: Mme Dorta, prof de système d'exploitation à Paris Descartes (Paris V).

### 2.1 Questions : Déploiement du projet et entrées sorties

* Comment compiler puis exécuter son programme ? Exécuter les tests ? Où sont rangés les binaires(en mode debug) ?

Compiler : `cargo build [OPTIONS]`

Exécuter : `cargo run [OPTIONS] [-- ARGS]`

Tester : `cargo test [OPTIONS] [TESTNAME] [-- TEST-OPTIONS]`

les binaires sont rangés dans :
 * target/debug/build

### 3.1 Questions : Exécuter une commande

3. Afficher le statut d'une commande, pourquoi Rust vous force à récupérer le statut ?

Le statut nous force à récupérer les résultats directement. Il redirige les résultats de notre nouveau processus à la sortie standard et tue le processus fils. Rust force ce comportement pour éviter d'avoir des processus zombis qui ont fini leur travail mais qui ne sont pas encore tués car ils attendent de transmettre leurs résultats.

4. Que fait votre programme pendant que son enfant s'exécute ?

S'ils sont liés il attend la réponse du fils; si non il continue sont travail et si jamais il fini alors que le fils n'a pas fini, le fils devient demon 

### 4.1 Questions : Redirections 

7. Un tube entre 2 programmes A et B est une sorte de branchemet entre la sortie standard du Programme A et l'entrée standard du programme B

### 5.1 Questions : Concurrence 

10. C’est quoi un processus ID ?

 Un PID : est un entier unique supérieur à zéro attribué par le système à tout processus pour l'identifier. Lorsqu'un processus se termine et n'est pas dans un état zombi le numéro PID qui lui a êté atribué peut être de nouveau utilisé par le système pour identifer un processus après un délai de garde.
