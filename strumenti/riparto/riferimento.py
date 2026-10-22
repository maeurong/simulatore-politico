"""Riparto di riferimento della legge 180/2026, scritto dal testo della legge per ricavare
i numeri attesi dei test di `crates/sim-core/tests/elezioni.rs` (piano 2 della v1).

Seconda lettura, indipendente dal codice Rust: se i due divergono si torna al testo della
legge (docs/ricerca/modelli/sistemi-elettorali.md:41-63), non a questo file. Le assunzioni
A1-A13 sono quelle del piano docs/superpowers/plans/2026-10-22-v1-piano-2-riparto-elettorale.md.
Solo libreria standard. Uso: python3 -I strumenti/riparto/riferimento.py
"""
CAM=[("piemonte_1","piemonte",2208370,0),("piemonte_2","piemonte",2047980,0),("lombardia_1","lombardia",4010096,0),("lombardia_2","lombardia",2099602,0),("lombardia_3","lombardia",2205257,0),("lombardia_4","lombardia",1628049,0),("veneto_1","veneto",1913482,0),("veneto_2","veneto",2934263,0),("friuli_venezia_giulia","friuli_venezia_giulia",1194647,0),("liguria","liguria",1509227,0),("emilia_romagna","emilia_romagna",4425366,0),("toscana","toscana",3663191,0),("umbria","umbria",858812,0),("marche","marche",1487150,0),("lazio_1","lazio",3822459,0),("lazio_2","lazio",1892423,0),("abruzzo","abruzzo",1275950,0),("molise","molise",292150,0),("campania_1","campania",2988376,0),("campania_2","campania",2636044,0),("puglia","puglia",3922941,0),("basilicata","basilicata",541168,0),("calabria","calabria",1855454,0),("sicilia_1","sicilia",2268634,0),("sicilia_2","sicilia",2564695,0),("sardegna","sardegna",1587413,0),("valle_d_aosta","valle_d_aosta",123360,1),("trentino_alto_adige","trentino_alto_adige",1073574,4)]
SEN=[("piemonte","piemonte",4256350,0,None),("valle_d_aosta","valle_d_aosta",123360,1,1),("lombardia","lombardia",9943004,0,None),("bolzano","trentino_alto_adige",532616,3,None),("trento","trentino_alto_adige",540958,3,None),("veneto","veneto",4847745,0,None),("friuli_venezia_giulia","friuli_venezia_giulia",1194647,0,None),("liguria","liguria",1509227,0,None),("emilia_romagna","emilia_romagna",4425366,0,None),("toscana","toscana",3663191,0,None),("umbria","umbria",858812,0,None),("marche","marche",1487150,0,None),("lazio","lazio",5714882,0,None),("abruzzo","abruzzo",1275950,0,None),("molise","molise",292150,0,2),("campania","campania",5624420,0,None),("puglia","puglia",3922941,0,None),("basilicata","basilicata",541168,0,None),("calabria","calabria",1855454,0,None),("sicilia","sicilia",4833329,0,None),("sardegna","sardegna",1587413,0,None)]
MACRO={"piemonte":"n","valle_d_aosta":"n","liguria":"n","lombardia":"n","trentino_alto_adige":"n","veneto":"n","friuli_venezia_giulia":"n","emilia_romagna":"n","toscana":"c","umbria":"c","marche":"c","lazio":"c","abruzzo":"m","molise":"m","campania":"m","puglia":"m","basilicata":"m","calabria":"m","sicilia":"m","sardegna":"m"}
REGIONI=list(dict.fromkeys(r for _,r,_,_ in CAM))

def hare_nat(pops,n):
    tot=sum(pops)
    if tot==0 or n==0: return [0]*len(pops)
    q=[p*n//tot for p in pops]; r=[p*n%tot for p in pops]
    for i in sorted(range(len(pops)),key=lambda i:(-r[i],-pops[i],i))[:n-sum(q)]: q[i]+=1
    return q

def seggi_ramo(units,totale,estero,minimo,premio):
    n=totale-estero
    fixed={u[0]:u[4] for u in units if len(u)>4 and u[4] is not None}
    while True:
        free=[u for u in units if u[0] not in fixed]
        s=hare_nat([u[2] for u in free], max(0,n-sum(fixed.values())))
        low=[u[0] for u,v in zip(free,s) if v<minimo]
        if not low: break
        for k in low: fixed[k]=minimo
    seg=dict(fixed); seg.update({u[0]:v for u,v in zip(free,s)})
    ordin=[u for u in units if u[3]==0]
    pr=hare_nat([u[2] for u in ordin],premio)
    prd={u[0]:v for u,v in zip(ordin,pr)}
    return [dict(id=u[0],regione=u[1],seggi=seg[u[0]],premio=prd.get(u[0],0),unin=min(u[3],seg[u[0]])) for u in units]

def ripartisci(formula,cifre,S):
    """-> (seggi, resti o None, resto_usato)"""
    n=len(cifre); T=sum(cifre)
    if S==0 or n==0 or T==0: return [0]*n,[None]*n,[False]*n
    if formula=="dhondt":
        s=[0]*n
        for _ in range(S):
            # massimo v/(s+1); a parità la cifra più alta, poi l'indice più basso
            best=None
            for i in range(n):
                if best is None or cifre[i]*(s[best]+1) > cifre[best]*(s[i]+1) or (cifre[i]*(s[best]+1)==cifre[best]*(s[i]+1) and cifre[i]>cifre[best]):
                    best=i
            s[best]+=1
        return s,[None]*n,[False]*n
    q=T//S
    if q>0:
        interi=[c//q for c in cifre]; resti=[c%q for c in cifre]
    if q==0 or sum(interi)>S:
        interi=[c*S//T for c in cifre]; resti=[c*S%T for c in cifre]
    s=interi[:]; resto_usato=[False]*n
    for i in sorted(range(n),key=lambda i:(-resti[i],-cifre[i],i))[:S-sum(interi)]:
        s[i]+=1; resto_usato[i]=True
    return s,resti,resto_usato

def riparto(legge,voti,coal_decl,quale):
    """quale: 'camera' o 'senato'"""
    nL=len(next(iter(voti.values())))
    naz=[sum(voti[r][l] for r in REGIONI) for l in range(nL)]; T=sum(naz)
    coal=[c for c in coal_decl if len(c)>=2] if legge["coalizioni"] else []
    in_coal={l:k for k,c in enumerate(coal) for l in c}
    reg20=quale=="senato" and legge["soglia_lista_pb"]>0
    def ok_lista(l):
        if naz[l]*10000>=legge["soglia_lista_pb"]*T: return True
        if reg20:
            for r in REGIONI:
                tr=sum(voti[r])
                if tr and voti[r][l]*10000>=2000*tr: return True
        return False
    soggetti=[]  # (tipo,idx, liste_ammesse)
    for k,c in enumerate(coal):
        cif=sum(naz[l] for l in c)
        if cif*10000>=legge["soglia_coalizione_pb"]*T and any(ok_lista(l) for l in c):
            amm=[l for l in c if ok_lista(l)]
            rest=[l for l in c if l not in amm]
            if legge["ripescaggio"] and rest:
                amm.append(max(rest,key=lambda l:(naz[l],-l)))
            soggetti.append(("C",k,sorted(amm),c))
    coal_ammesse={s[1] for s in soggetti}
    for l in range(nL):
        if (l not in in_coal or in_coal[l] not in coal_ammesse) and ok_lista(l):
            soggetti.append(("L",l,[l],[l]))
    cifra=lambda s: sum(naz[l] for l in s[3])
    return soggetti, naz, T, coal

def golden(legge, voti, coal_decl):
    res={}
    nL=len(next(iter(voti.values())))
    sogg={}
    for q in ("camera","senato"):
        s,naz,T,coal=riparto(legge,voti,coal_decl,q); sogg[q]=(s,naz,T,coal)
    # premio
    premio=None
    if legge["premio"]!="nessuno":
        tops=[]
        for q in ("camera","senato"):
            s,naz,T,_=sogg[q]
            if not s: tops.append(None); continue
            cif=[sum(naz[l] for l in x[3]) for x in s]
            i=max(range(len(s)),key=lambda i:(cif[i],-i))
            tops.append((s[i][0],s[i][1]) if cif[i]*10000>=legge["soglia_premio_pb"]*T else None)
        if tops[0] is not None and tops[0]==tops[1]: premio=tops[0]
    out={}
    for q,units,tot,est,mn in (("camera",CAM,400,8,0),("senato",SEN,200,4,3)):
        s,naz,T,coal=sogg[q]
        P=(legge["taglia_premio_pb"]*tot+5000)//10000 if premio else 0
        seg=seggi_ramo(units,tot,est,mn,P)
        L=[dict(prop=0,premio=0,unin=0,estero=0) for _ in range(nL)]
        perreg={r:[0]*nL for r in REGIONI}
        key=lambda x:(x[0],x[1])
        win=[i for i,x in enumerate(s) if key(x)==premio]
        w=win[0] if win else None
        # soggetti degli uninominali: coalizioni dichiarate (se ammesse) più liste sole
        usogg=[c for c in coal]+[[l] for l in range(nL) if all(l not in c for c in coal)]
        sp_unin=[0]*len(s)  # seggi per soggetto nelle speciali (contano per il tetto)
        def give_unin(reg,n):
            v=voti[reg]
            cs,_,_=ripartisci("dhondt",[sum(v[l] for l in c) for c in usogg],n)
            for c,k in zip(usogg,cs):
                if not k: continue
                ls,_,_=ripartisci("dhondt",[v[l] for l in c],k)
                for l,kk in zip(c,ls):
                    L[l]["unin"]+=kk; perreg[reg][l]+=kk
                    for i,x in enumerate(s):
                        if l in x[3] and (x[0]=="C" or x[1]==l):
                            sp_unin[i]+=kk
        def split_in_coal(i,n,cifre_l,campo,reg=None):
            x=s[i]; amm=x[2]
            ls,_,_=ripartisci(legge["formula"],[cifre_l[l] for l in amm],n)
            for l,k in zip(amm,ls):
                L[l][campo]+=k
                if reg: perreg[reg][l]+=k
        if q=="camera":
            # speciali
            for u in seg:
                if u["unin"]>0 or u["seggi"]==0 and False: pass
            spec=[u for u in seg if [x for x in CAM if x[0]==u["id"]][0][3]>0]
            for u in spec:
                give_unin(u["regione"],u["unin"])
                rest=u["seggi"]-u["unin"]
                if rest>0 and s:
                    v=voti[u["regione"]]
                    cs,_,_=ripartisci(legge["formula"],[sum(v[l] for l in x[3]) for x in s],rest)
                    for i,k in enumerate(cs):
                        sp_unin[i]+=k
                        if k: split_in_coal(i,k,v,"unin",u["regione"])
            N=sum(u["seggi"]-u["premio"] for u in seg if u not in spec)
            senza=[naz[l]-sum(voti[u["regione"]][l] for u in spec) for l in range(nL)]
            cif=[sum(senza[l] for l in x[3]) for x in s]
            cs,_,_=ripartisci(legge["formula"],cif,N)
            tetto=False
            if w is not None and legge["tetto"] is not None:
                C=(legge["tetto"][0]*tot+5000)//10000
                if P+cs[w]+sp_unin[w]>C:
                    tetto=True
                    cs[w]=max(0,C-P-sp_unin[w])
                    oth=[i for i in range(len(s)) if i!=w]
                    o,_,_=ripartisci(legge["formula"],[cif[i] for i in oth],N-cs[w])
                    for i,k in zip(oth,o): cs[i]=k
            for i,k in enumerate(cs):
                if k: split_in_coal(i,k,senza,"prop")
            if w is not None and P:
                split_in_coal(w,P,naz,"premio")
            out_t=tetto
        else:
            spec=[u for u in seg if u["unin"]>0]
            for u in spec: give_unin(u["regione"],u["unin"])
            # riparto regionale
            regs=[u for u in seg if u["unin"]==0]
            alloc={}
            for u in regs:
                v=voti[u["regione"]]
                cif=[sum(v[l] for l in x[3]) for x in s]
                cs,resti,da=ripartisci(legge["formula"],cif,u["seggi"]-u["premio"])
                alloc[u["regione"]]=dict(cs=cs,resti=resti,da=da,cif=cif,used=[d for d in da])
            tetto=False
            if w is not None and legge["tetto"] is not None:
                C=(legge["tetto"][1]*tot+5000)//10000
                totw=P+sum(a["cs"][w] for a in alloc.values())+sp_unin[w]
                exc=totw-C
                if exc>0:
                    tetto=True
                    order=list(alloc.keys())
                    def cut(r):
                        a=alloc[r]
                        oth=[i for i in range(len(s)) if i!=w]
                        if not oth: return False
                        cand=[i for i in oth if a["resti"][i] is not None and not a["used"][i]]
                        if cand:
                            j=max(cand,key=lambda i:(a["resti"][i],a["cif"][i],-i)); a["used"][j]=True
                        else:
                            j=max(oth,key=lambda i:(a["cif"][i],-i))
                        a["cs"][w]-=1; a["cs"][j]+=1; return True
                    tagliate=set()
                    fa=[r for r in order if alloc[r]["da"][w]]
                    fa.sort(key=lambda r:(alloc[r]["resti"][w],order.index(r)))
                    for r in fa:
                        if exc==0: break
                        if cut(r): exc-=1; tagliate.add(r)
                    def pct_key(r):
                        a=alloc[r]; tr=sum(voti[r]); return (a["cif"][w]/tr if tr else 0, order.index(r))  # solo riferimento (qui il float va bene)
                    first=True
                    while exc>0:
                        moved=False
                        for r in sorted(order,key=pct_key):
                            if exc==0: break
                            if first and r in tagliate: continue
                            if alloc[r]["cs"][w]<=1: continue
                            if cut(r): exc-=1; moved=True
                        first=False
                        if not moved: break
            for r,a in alloc.items():
                for i,k in enumerate(a["cs"]):
                    if k: split_in_coal(i,k,voti[r],"prop",r)
            if w is not None and P:
                split_in_coal(w,P,naz,"premio")
            out_t=tetto
        # estero
        amm_l=sorted(l for x in s for l in x[2])
        es,_,_=ripartisci(legge["formula"],[naz[l] for l in amm_l],est)
        for l,k in zip(amm_l,es): L[l]["estero"]+=k
        out[q]=dict(liste=L,tetto=out_t,perreg=perreg if q=="senato" else None)
    out["premio"]=premio
    return out

L180=dict(soglia_lista_pb=300,coalizioni=True,soglia_coalizione_pb=1000,ripescaggio=True,premio="coalizione",soglia_premio_pb=4200,taglia_premio_pb=1750,tetto=(5500,5650),formula="hare")
P3=dict(soglia_lista_pb=300,coalizioni=False,soglia_coalizione_pb=0,ripescaggio=False,premio="nessuno",soglia_premio_pb=4000,taglia_premio_pb=875,tetto=None,formula="hare")
PP=dict(P3,soglia_lista_pb=0)

POP={}
for _,r,p,_ in CAM: POP[r]=POP.get(r,0)+p
def voti_da(tab, override=None):
    # tab: {macro: [per mille per lista]}; voti = pop/2 * pm /1000
    v={}
    for r in REGIONI:
        pm=tab[MACRO[r]]
        if override and r in override: pm=override[r]
        v[r]=[POP[r]//2*x//1000 for x in pm]
    return v

COAL=[[0,1,2,3],[4,5,6]]
G1=dict(c=[300,90,25,10,220,60,20,140,50,20,65],n=[320,90,25,10,200,60,20,140,50,20,65],m=[280,90,25,10,240,60,20,140,50,20,65])
G1_OV={"sardegna":[280,90,25,10,240,60,20,0,50,225,0]}
G2=dict(n=[400,90,25,10,150,50,20,120,50,20,65],c=[380,90,25,10,170,50,20,120,50,20,65],m=[360,90,25,10,190,50,20,120,50,20,65])
def show(nome,legge,v):
    o=golden(legge,v,COAL)
    naz=[sum(v[r][l] for r in REGIONI) for l in range(11)]; T=sum(naz)
    print("==",nome,"premio",o["premio"],"tetto camera",o["camera"]["tetto"],"senato",o["senato"]["tetto"])
    print(" quote naz ‰", [round(x*1000/T,2) for x in naz], "coal0", round(sum(naz[:4])*100/T,3),"coal1",round(sum(naz[4:7])*100/T,3))
    for q in ("camera","senato"):
        L=o[q]["liste"]
        print(" ",q,"tot",[x["prop"]+x["premio"]+x["unin"]+x["estero"] for x in L], "somma", sum(x["prop"]+x["premio"]+x["unin"]+x["estero"] for x in L))
        print("    prop",[x["prop"] for x in L]); print("    prem",[x["premio"] for x in L]); print("    unin",[x["unin"] for x in L]); print("    est ",[x["estero"] for x in L])
        if q=="senato": print("    coal0 senza estero", sum(L[l]["prop"]+L[l]["premio"]+L[l]["unin"] for l in range(4)))
        else: print("    coal0 senza estero", sum(L[l]["prop"]+L[l]["premio"]+L[l]["unin"] for l in range(4)))
    return o
def rust(nome,legge,v):
    o=golden(legge,v,COAL)
    for q in ("camera","senato"):
        L=o[q]["liste"]
        print(f"// {nome} {q}")
        for f,lab in (("prop","proporzionali"),("premio","premio"),("unin","speciali"),("estero","estero")):
            print(f"  {lab}: {[x[f] for x in L]}")
v1=voti_da(G1,G1_OV); v2=voti_da(G2,G1_OV)
rust("G1 legge_180",L180,v1); rust("G2 legge_180",L180,v2); rust("G1 proporzionale_3",P3,v1); rust("G1 proporzionale_puro",PP,v1); rust("G2 dhondt",dict(L180,formula="dhondt"),v2); rust("G2 senza tetto",dict(L180,tetto=None),v2)
show("G1 legge180",L180,v1); show("G2 legge180 tetto",L180,v2); show("G1 prop3",P3,v1); show("G1 puro",PP,v1)
import copy
D=dict(L180,formula="dhondt"); show("G2 dhondt",D,v2)
