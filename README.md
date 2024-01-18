# pictochar ✏️
simple cli tool to draw ascii in the terminal 😁👍\
written in ~~poor~~ rust btw 🦀
## usage
`pictochar [OPTIONS] <FILE>`

arguments:
- `<FILE>`  a path or link to the image file 📁🔗

options:
- `-m`, `--mode <MODE>` color mode 🎨 (default: intensity, possible values: red, green, blue, alpha, saturation, intensity, value, luminance, hue)
- `-g`, `--gradient <GRADIENT>` ascii art gradient 🌇 (must be at least 2 characters, default: ` .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$`)
- `-b`, `--blur <BLUR>` gaussian blur sigma 🗿🚬 (default: 0.5)
- `-i`, `--invert` inverts image's colors 🌃
- `-a`, `--aspect-ratio <ASPECT_RATIO>` image size multiplier 🖼️ (default: 1)
- `-h`, `--help` print help 🆘
- `-v`, `--version` print version 👴

## example
```
$ cargo run -- https://ic.pics.livejournal.com/tema/339052/2572897/2572897_1000.jpg -a 0.05 -i
     .   ,_(tftnxcvYUJJUufjuunX]        
      >jxfuX\?-+>>l!i!l;!!`   ,uf       
    "uu_?!^"I>~+~<+<i>>i;l+~!'  tf .    
   iC+  ^l][-!:,:,.i]<+?}-i~~+l  c?     
  'U^  ._i::l+I    -l.''"<-I!<+! `J     
  !f ' ],    ~`   ^-       +Il<^. j} .  
. )\   ; ..  <l   .'!rv0wY|.I.    .Y  . 
 -Y''^.^UXzO/:     1vl.d$$kO>^l' <_zb^ .
}U!~+<!!0uza$d{I  ~h")cxftrk|>\^^{+~Lq, 
J!-+.     ..:dO'   /U[' ?i   l[nj\n, xd'
X~++zrY~ " . [+ .       :cv)fzx:  !Q^'QX
Q_<`, iwY[  IC;    `      l~i  _k' ?f |Q
L<+, |" '  \0l     <X\<,;`   [Y|zb<^r )L
01+!"8! '![8v    ]jl_X^II!lrOt, fx;}/ /m
]n^!{$d ;l. xt,` ~~^>i -/vfYr  \b  !I+0t
 zl v$J*/'   [r"  `>1Cw|~  )Y1m$_  l}zJ 
 ]/ Jq;j-aXrf/\fXLnf_\r  In$M_\v . ^um. 
 if Yhc{ X .+ql })    Ork$Ow~ J    lU.  
 if c$$*qh)~)O<>nU~1Ua$Mv! IJUI    UI . 
 it n$@$$$$$$$$$$$$Bb1c~   ~aI .. ~v .  
 if ?*o#BW$%&$hdYqf   <r  tY.    .L:    
 i/ .0a:Y'0_.Ci  [i    QrX1 "."'`Q~ .   
 it  <o)p}[n )[  (1 :-t0r` <?<!-Cl .    
 ]|   .[zXYq|CZ)(C0fj\>  >_+!ifJ: .     
 /~ l -^  `!+iI!<l   .;-[<l>|X}  .      
 /! ]; <~;:<~_?][[~-1|{>;?rz|           
 tl  ~+I!+_~~~+___??!  _vx_.  .         
 (| . 'Ii!!iiiil:.   ?Xxl   .           
  C>             l{|Yj:  .              
  "Y\:      `i{rvn/{"  .                
    [cXxxxxcXn)i`    .                  
```
