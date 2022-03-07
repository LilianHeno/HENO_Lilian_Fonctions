fn pgcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn test_nb_premier(n: i32) {
    let mut i = 2;
    while i < n && n % i != 0 {
        i = i + 1;
        if i == n {
            return println!("{} est premier", n);
        }
    }
    return println!("{} n'est pas premier", n);
}


fn factoriel(n:i32) -> i32 {
    let mut resultat = 1;
    for x in 2..n+1 {
        resultat*=x;
    }
    resultat
}

fn nbcombinaison(n:i32, k:i32)->i32{
    return factoriel(n)/factoriel(k)*(factoriel(n-k));
}

fn  nbarrangement(n:i32,k:i32)-> i32{
    return  factoriel(n)/(factoriel(n-k));
}


fn celcius_fahr(degrecelcius: i32) -> i32 {
    return (degrecelcius * 9 / 5) + 32;
}

fn fibonacci(n:i32)->i32{
    let mut temp = 1;
    let mut v =0;
    for _i in 0..n{
        v = v+temp;
        temp = v-temp;
    }
    return v;
}


fn main() {
    println!("Le pgcd de {} et {} est {}",1896,12223,pgcd(1896,12223));
    test_nb_premier(653231);
    println!("{} degrés celcius correspond à {} degrés fahrenheit",250,celcius_fahr(250));
    println!("La {} valeur de la suite de fibonacci est {}",138,fibonacci(7));
    println!("{}",nbcombinaison(5,2));
    println!("Le nombre d'arrangement est {}",nbarrangement(10,4));
    println!("La valeur factoriel de 5 est {}",factoriel(10));
}


