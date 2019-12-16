fn compute_fft(digits: &Vec<u8>) -> Vec<u8> {
    let mut partial_sums: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    partial_sums.push(sum);
    for digit in digits {
        sum += *digit as i32;
        partial_sums.push(sum);
    }

    let mut new_digits: Vec<u8> = Vec::new();
    for i in 1..digits.len() + 1 {
        let mut sum: i32 = 0;
        for j in (i - 1..digits.len()).step_by(4 * i) {
            sum += partial_sums[digits.len().min(j + i)] - partial_sums[j];
        }

        for j in (3 * i - 1..digits.len()).step_by(4 * i) {
            sum -= partial_sums[digits.len().min(j + i)] - partial_sums[j];
        }

        new_digits.push((sum % 10).abs() as u8);
    }

    new_digits
}

fn main() {
    let input = "59787832768373756387231168493208357132958685401595722881580547807942982606755215622050260150447434057354351694831693219006743316964757503791265077635087624100920933728566402553345683177887856750286696687049868280429551096246424753455988979991314240464573024671106349865911282028233691096263590173174821612903373057506657412723502892841355947605851392899875273008845072145252173808893257256280602945947694349746967468068181317115464342687490991674021875199960420015509224944411706393854801616653278719131946181597488270591684407220339023716074951397669948364079227701367746309535060821396127254992669346065361442252620041911746738651422249005412940728".repeat(10000);
    let offset = 5978783;
    let mut digits: Vec<u8> = input
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();

    for i in 0..100 {
        println!("{} phases completed", i + 1);
        digits = compute_fft(&digits);
    }

    println!(
        "{:?}",
        digits[offset..offset + 8]
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
