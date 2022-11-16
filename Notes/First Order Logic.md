# First Order Logic
[Propositional Logic](Notes/Propositional%20Logic.md) can only deal with a finite number of propositions:

T: Tommy is faithful
J: Jimmy is faithful
L: Laika is faithful
$All\ dogs\ are\ faithful\iff T\land J\land L$ 
What if there is an infinite/unknown number of dogs?

![](https://i.imgur.com/cv3VDgZ.png)

### Forming FOL sentences
“All dogs are mammals"
General form:  $\forall xDog(x)\implies Mammal(x)$
Use conjunction? $\forall x Dog(x) \land Mammal(x)$ : **this is means everything is a dog and a mammal!**

"John owns a dog"
General form: $\exists x Dog(x)\land Owns(John,x)$
Use implication? $\exists xDog(x)\implies Owns(John,x)$: **this can mean that John owns things which are not dogs as well**

### Inference Rules
![](https://i.imgur.com/u2fblRM.png)

![](https://i.imgur.com/SEjFjs7.png)

Using substitutions is also called _Generalized Modus Ponens_.
The substitution used is called the _unifier_.

### Getting to CNF
![](https://i.imgur.com/oQcOri5.png)
![](https://i.imgur.com/Ao7wmqA.png)



$$\begin{align}
  \exists xStudent(x)\land \neg Takes(x,AI)\tag1\equiv Student(K)\land\neg Takes(K,AI) \\
  \tag2 \exists xStudent(x)\land Takes(x,AI)\land\neg pass(x,AI)\equiv \\
  Student(F)\land Takes(F,AI)\land \neg pass(F,AI)\tag3\\
   \forall x,y \neg Student(x)\lor\neg pass(x,y)\lor\neg hard(y)\lor diligent(x)\models\\\tag4\neg Student(x)\lor\neg pass(x,y)\lor\neg hard(y)\lor diligent(x) \\
   3+4:\neg pass(x,y)\lor\neg hard(y)\lor diligent(x)\tag5 \\
   \tag6 3+4+5: Takes(x,AI)\lor\neg hard(AI)\\
   6+Subst\{x/K\}\ Takes(K,AI)\lor\neg hard(y)\tag7\\
   1+7: \neg hard(AI)\tag8\\
   8+iv:\emptyset
  \end{align}$$
