
pub fn generate_address(redeem_script: &[u8]) -> Vec<u8> {
  let hash160 = ripemd160(redeem_script);

    // Prefix with 0x05 for P2SH addresses (mainnet)
    let mut address_bytes = vec![0x05];
    address_bytes.extend_from_slice(&hash160);

    address_bytes
}




fn ripemd160(input: &[u8]) -> Vec<u8> {
  // Constants for RIPEMD160
  let mut h0: u32 = 0x67452301;
  let mut h1: u32 = 0xefcdab89;
  let mut h2: u32 = 0x98badcfe;
  let mut h3: u32 = 0x10325476;
  let mut h4: u32 = 0xc3d2e1f0;

  // Initialize constants for the compression function
  let k: [u32; 5] = [0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xa953fd4e, 0x50a28be6];

  // Pad the input with a single '1' bit followed by zeros, then the length in bits
  let mut padded_input = input.to_vec();
  padded_input.push(0x80);
  while (padded_input.len() * 8) % 512 != 448 {
      padded_input.push(0);
  }
  let input_bits = (input.len() * 8) as u64;
  padded_input.extend_from_slice(&input_bits.to_le_bytes());

  // Process blocks of 512 bits
  for chunk in padded_input.chunks_exact(64) {
      let mut words = [0u32; 16];

      // Convert chunk into array of u32 words
      for i in 0..16 {
          words[i] = u32::from_le_bytes([
              chunk[i * 4],
              chunk[i * 4 + 1],
              chunk[i * 4 + 2],
              chunk[i * 4 + 3],
          ]);
      }

      // Compression function
      let mut a = h0;
      let mut b = h1;
      let mut c = h2;
      let mut d = h3;
      let mut e = h4;

      // Round 1
      for i in 0..16 {
          let f = (b & c) | ((!b) & d);
          let temp = e.wrapping_add(f).wrapping_add(words[i]).wrapping_add(k[0]);
          e = d;
          d = c;
          c = b.rotate_left(10);
          b = a;
          a = temp;
      }

      // Round 2
      for i in 0..16 {
          let f = (b & c) | (b & d) | (c & d);
          let temp = e.wrapping_add(f).wrapping_add(words[(5 * i + 1) % 16]).wrapping_add(k[1]);
          e = d;
          d = c;
          c = b.rotate_left(10);
          b = a;
          a = temp;
      }

      // Round 3
      for i in 0..16 {
          let f = b ^ c ^ d;
          let temp = e.wrapping_add(f).wrapping_add(words[(3 * i + 5) % 16]).wrapping_add(k[2]);
          e = d;
          d = c;
          c = b.rotate_left(10);
          b = a;
          a = temp;
      }

      // Round 4
      for i in 0..16 {
          let f = c ^ (b | (!d));
          let temp = e.wrapping_add(f).wrapping_add(words[(7 * i) % 16]).wrapping_add(k[3]);
          e = d;
          d = c;
          c = b.rotate_left(10);
          b = a;
          a = temp;
      }

      // Round 5
      for i in 0..16 {
          let f = b ^ (c | (!d));
          let temp = e.wrapping_add(f).wrapping_add(words[i % 16]).wrapping_add(k[4]);
          e = d;
          d = c;
          c = b.rotate_left(10);
          b = a;
          a = temp;
      }

      // Update hash values
      h0 = h0.wrapping_add(a);
      h1 = h1.wrapping_add(b);
      h2 = h2.wrapping_add(c);
      h3 = h3.wrapping_add(d);
      h4 = h4.wrapping_add(e);
  }

  // Concatenate hash values
  let mut hash_bytes = Vec::new();
  hash_bytes.extend_from_slice(&h0.to_be_bytes());
  hash_bytes.extend_from_slice(&h1.to_be_bytes());
  hash_bytes.extend_from_slice(&h2.to_be_bytes());
  hash_bytes.extend_from_slice(&h3.to_be_bytes());
  hash_bytes.extend_from_slice(&h4.to_be_bytes());

  // Truncate to 20 bytes
  hash_bytes.truncate(20);

  hash_bytes
}