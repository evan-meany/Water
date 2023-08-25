Abstract Syntax Tree
$$
\begin{align}
 [\text{Program}] &\to [\text{Statement}]^* \\
 
 [\text{Statement}] &\to \begin{cases}
   \text{return [Expression];} \\
   \text{[Type] [Identifier] = [Expr];} \\
   \text{[Identifier] = [Expr];} \\
 \end{cases} \\
 
 [\text{Expression}] &\to \begin{cases}
   \text{[TypeLiteral]} \\
   \text{[Identifier]}
 \end{cases}

\end{align}
$$