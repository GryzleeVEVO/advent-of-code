import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

class day_4 {

    public static void main(String[] args) {
        try {
            BufferedReader br = new BufferedReader(new FileReader("input.txt"));
            String key = br.readLine();

            if (key != null) {
                int idx = 0;
                int idx_1 = 0;
                int idx_2 = 0;
                MessageDigest md = MessageDigest.getInstance("MD5");

                while (idx_1 == 0 | idx_2 == 0) {
                    // Get new key
                    String inputKey = key + Integer.toString(idx);

                    // Create digest
                    byte[] digest = md.digest(inputKey.getBytes());

                    // Does the hash start with five zeroes (first 2B+4b)?
                    if (idx_1 == 0 && digest[0] == 0 &&
                            digest[1] == 0 &&
                            (digest[2] & 0b11110000) == 0) {
                        idx_1 = idx;
                    }

                    // Does the hash start with five zeroes (first 3B)?
                    if (idx_2 == 0 && digest[0] == 0 &&
                            digest[1] == 0 &&
                            digest[2] == 0) {
                        idx_2 = idx;
                    }

                    idx++;
                }

                System.out.println("Part 1: " + Integer.toString(idx_1));
                System.out.println("Part 2: " + Integer.toString(idx_2));
            }

            br.close();
        } catch (IOException | NoSuchAlgorithmException e) {
            e.printStackTrace();
        }

    }

}