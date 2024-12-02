fn difference(x: u32,y:u32) -> u32 {
    if x > y {
        x-y
    } else {
        y-x
    }
}

fn check_line(numbers: &Vec<u32>) -> bool {
    let mut mode = 0;
    // 0: no checked; 1: increasing; 2: decreasing
    for idx in 1..(numbers.len()) {
        let current = numbers[idx];
        let previous = numbers[idx - 1];
        if current > previous {
            if mode == 0 {
                mode = 1;
            } else if mode == 2 {
                // line not safe
                return false;
            }
        } else if current < previous {
            if mode == 0 {
                mode = 2;
            } else if mode == 1 {
                // line not safe
                return false;
            }
        }

        let difference = difference(current, previous);
        if difference < 1 {
            // line not safe
            return false;
        } else if difference > 3 {
            // line not safe
            return false;
        }
    }
    true
}

pub fn run() {
    let lines: Vec<&str> = input().lines().collect();
    let mut safe_lines_first:u32 = 0;
    let mut safe_lines_second:u32 = 0;

    for line in lines.iter() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let numbers = words.iter().map(|&x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        
        if check_line(&numbers) {
            safe_lines_first += 1;
        } else {
            for remove_idx in 0..numbers.len() {
                let mut new_numbers = numbers.clone();
                new_numbers.remove(remove_idx);
                if check_line(&new_numbers) {
                    safe_lines_second += 1;
                    break;
                }
            }
        }
    }
    println!("Ex2: Safe lines first: {}", safe_lines_first);
    println!("Ex2: Safe lines second: {}", safe_lines_first+safe_lines_second);
}

fn test() -> &'static str {
    "2 1 2 3 4
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
}

fn input() -> &'static str {
    "1 4 5 8 11 12 9
7 8 9 10 12 15 17 17
17 20 23 25 27 31
55 57 58 61 63 64 70
39 42 45 43 44
84 85 86 87 90 89 86
33 34 35 36 35 37 38 38
8 9 11 8 10 11 13 17
34 35 37 39 38 40 45
47 50 50 51 53 54
54 55 58 58 59 56
95 98 99 99 99
53 54 54 57 61
65 68 71 71 73 78
19 20 23 27 28 30 33 36
78 79 81 85 83
24 25 27 29 30 32 36 36
78 79 80 83 86 90 93 97
30 31 35 38 40 42 49
61 63 66 71 74 75
77 80 82 83 89 87
58 61 68 70 71 74 75 75
20 23 24 25 28 35 36 40
25 28 35 36 38 40 46
47 46 49 52 55
78 76 78 79 81 78
14 13 15 17 19 21 21
77 74 77 79 80 84
91 89 91 93 94 99
65 64 63 65 66 67 70
49 48 51 53 56 54 52
29 28 30 33 31 34 34
70 69 71 70 74
41 38 39 41 43 41 46
88 85 88 90 90 92
37 34 36 37 37 39 42 40
57 55 58 61 61 61
46 44 46 46 50
78 75 77 78 79 79 82 89
86 83 86 87 91 92 95
17 14 16 17 21 18
34 32 36 39 39
60 59 62 66 68 70 73 77
59 58 60 61 63 65 69 75
41 40 42 45 46 49 55 58
4 3 4 5 8 15 16 14
5 4 10 11 11
15 14 17 24 26 30
59 56 59 60 67 69 70 77
38 38 39 41 43
78 78 81 84 85 88 85
24 24 25 27 29 30 31 31
51 51 54 55 56 58 61 65
30 30 32 35 40
84 84 85 82 83
45 45 48 49 46 43
60 60 59 60 61 62 62
35 35 36 33 36 39 43
47 47 44 45 52
85 85 86 86 88 91 94
55 55 55 57 56
24 24 24 27 29 31 32 32
47 47 50 52 52 54 55 59
41 41 41 43 49
7 7 10 12 16 19 20 23
28 28 30 34 37 38 40 39
72 72 74 77 80 84 86 86
25 25 27 29 32 36 39 43
15 15 18 22 28
79 79 82 89 92 93
55 55 60 61 59
23 23 24 31 33 33
58 58 64 65 69
76 76 83 84 87 89 92 97
50 54 55 57 59
23 27 28 30 31 32 33 31
8 12 14 17 20 20
31 35 37 40 42 43 46 50
51 55 57 58 63
37 41 44 47 45 46
23 27 28 31 29 32 29
68 72 75 72 72
39 43 45 47 44 45 47 51
20 24 21 24 30
74 78 79 80 80 81 82 83
58 62 64 64 67 68 69 67
17 21 23 23 26 26
21 25 27 29 30 30 34
86 90 92 92 98
23 27 29 30 33 37 39 40
33 37 38 42 44 43
29 33 36 37 40 44 44
21 25 27 28 32 35 39
56 60 62 66 67 73
69 73 80 82 83 85 87
22 26 31 34 35 38 41 39
40 44 50 52 52
45 49 54 57 58 60 64
41 45 46 49 51 58 63
46 53 55 56 59 62
10 16 18 20 21 22 21
19 24 26 29 29
28 34 36 39 42 44 48
10 17 20 21 23 26 32
79 86 88 87 88
8 13 14 15 13 16 13
18 25 26 28 31 32 29 29
19 24 21 22 26
41 48 50 47 53
58 65 68 68 69 72 74 75
84 90 90 93 95 93
63 68 68 69 69
77 84 84 86 89 93
9 14 16 16 17 20 27
17 22 25 29 32 35 37 40
76 83 87 89 86
30 37 39 41 42 46 46
76 83 84 85 89 91 95
76 82 86 87 88 89 96
68 75 80 81 83 85 88
1 7 10 11 16 15
82 87 90 96 96
70 75 77 82 85 89
18 25 27 30 33 39 41 46
23 20 18 17 14 11 8 9
98 96 95 92 91 90 90
78 77 76 74 73 69
52 51 50 47 44 42 39 33
82 80 82 79 78 77 74
41 38 39 37 40
35 32 34 32 32
81 78 75 72 73 71 68 64
64 63 62 61 60 61 55
19 17 16 13 13 10
23 22 20 20 21
52 50 49 48 45 45 44 44
19 16 15 12 10 10 6
25 24 24 23 22 20 13
95 92 91 89 87 86 82 79
54 51 49 46 42 44
73 71 69 67 63 61 61
82 80 77 73 72 70 66
23 20 17 13 12 11 6
33 30 29 22 21
18 16 9 8 6 5 7
89 86 85 80 80
54 52 47 46 45 41
73 71 70 68 66 65 59 53
57 58 57 54 51
88 89 86 84 82 79 76 78
42 45 42 41 39 38 38
74 77 76 74 70
21 23 22 19 14
15 18 17 20 18 17 16 13
49 50 52 51 48 50
21 23 20 22 19 16 16
61 62 60 61 59 58 54
94 97 94 93 94 88
91 94 94 91 90
43 46 45 45 47
33 35 35 33 33
63 65 62 60 60 56
27 29 28 27 25 22 22 16
80 83 79 77 75 72
66 68 67 63 62 61 64
31 33 32 28 28
73 76 72 71 68 65 61
62 65 64 63 59 57 51
24 26 23 22 21 19 12 11
23 24 19 18 15 14 16
87 90 84 81 78 77 77
80 82 80 74 70
37 39 36 30 24
66 66 64 61 60 59
9 9 8 7 4 2 5
30 30 28 27 25 25
59 59 56 54 52 48
27 27 25 23 20 17 10
41 41 44 42 41 38 37
92 92 91 94 92 94
53 53 52 51 50 52 51 51
29 29 30 29 26 22
48 48 47 44 41 44 37
60 60 57 57 54
25 25 22 19 19 16 19
39 39 38 35 32 32 30 30
92 92 90 90 88 86 82
86 86 86 84 81 80 79 72
95 95 92 89 85 84 81 80
30 30 27 23 20 18 15 16
96 96 94 91 88 85 81 81
77 77 75 74 70 68 66 62
46 46 43 42 41 37 30
67 67 60 57 55 54 52 51
35 35 33 26 27
79 79 76 74 68 66 65 65
67 67 66 59 55
51 51 48 43 40 33
32 28 27 25 24
39 35 32 31 30 29 31
49 45 43 40 39 39
45 41 39 37 36 32
46 42 41 38 37 34 31 24
85 81 82 81 80
20 16 18 17 15 14 12 15
71 67 65 62 65 62 60 60
87 83 84 81 77
9 5 8 7 6 1
54 50 49 49 47 46 44
98 94 93 93 92 91 94
94 90 87 85 85 83 83
79 75 75 72 68
88 84 81 80 80 75
43 39 35 32 31 29
40 36 32 29 28 25 22 25
62 58 57 53 53
71 67 66 63 61 57 55 51
53 49 45 42 37
36 32 29 26 24 21 14 13
96 92 89 86 84 79 78 79
45 41 39 37 31 28 28
93 89 84 81 79 76 74 70
60 56 55 52 47 44 41 36
55 48 45 43 41 39 36
39 32 31 29 28 30
80 73 71 69 67 65 63 63
22 17 16 15 12 10 6
92 86 84 81 79 76 74 69
29 23 22 21 19 21 19
70 64 61 58 61 60 58 61
33 26 23 20 22 20 17 17
57 50 53 50 48 45 44 40
61 56 59 57 51
30 25 23 23 22
59 53 53 51 48 51
19 13 10 9 7 7 7
53 46 45 42 39 39 35
17 11 11 9 6 1
73 67 65 64 60 57 56 53
69 63 59 57 58
34 27 25 22 21 17 17
53 47 43 41 40 36
51 45 44 42 41 37 34 29
60 55 52 51 48 46 40 39
21 15 12 10 5 8
26 19 13 12 11 9 7 7
63 56 54 49 45
38 31 25 23 21 19 13
70 63 63 61 60 57 54 56
68 64 63 62 59 59 57
98 94 91 91 88 85 83 79
49 51 53 54 57 62 65 71
53 57 63 64 67 71
30 34 35 39 42 43 43
69 76 79 79 82 83 87
66 66 63 60 60 59 55
21 28 31 34 33 34
85 84 83 80 79 79 79
62 62 62 59 56
91 90 87 82 79 79
61 60 59 55 52
71 69 70 72 74 76 79 76
95 93 92 88 85 84 83 83
86 82 80 77 76 69
25 29 31 33 34 34 35 39
23 23 24 21 20 19 15
71 75 75 78 79
88 86 89 89 92
35 35 37 40 38 38
59 59 58 54 52 48
43 38 35 32 30 24 22 21
64 69 69 72 74
67 68 67 64 63 59 62
71 71 65 63 60 56
53 56 57 55 57 59 60 58
75 78 75 72 74 71 68 68
80 79 82 85 82 86
79 73 73 72 72
70 70 69 63 61 54
31 37 44 46 47 49 52 51
70 69 70 73 72 69
31 31 28 31 30 27 25 18
28 22 20 19 16 12
60 55 52 52 49 47 46 42
39 35 32 32 31 28 31
16 20 21 25 24
8 7 8 10 11 10 11
78 81 84 87 87 88
82 83 80 78 74
2 7 9 12 13 17 19 18
45 47 50 53 56 59 59
21 21 23 21 23
14 16 11 10 5
86 89 88 87 86 84 82 82
50 46 45 38 34
16 19 21 23 20 22
57 57 60 63 66 66 65
75 74 75 79 83
86 83 84 82 79 78 80
65 65 62 61 63 61 59 59
73 76 80 81 83 85 89
99 92 93 91 90 83
41 45 48 51 54 55 61
85 79 72 71 69 70
20 17 18 20 23 30 31 31
62 63 64 66 66 70
70 73 77 78 85
66 65 68 70 76 78 82
35 39 41 44 41 42 44
12 19 22 25 28 30 32 37
80 77 76 75 73 72 65
93 86 80 79 75
58 54 48 46 43 41
29 27 30 33 30 31 34 41
31 32 33 35 37 37 36
34 35 33 27 26 25 24
81 83 85 86 88 90 92 90
47 46 44 40 38 41
37 37 37 36 33 26
85 85 79 78 75 74 72 73
30 30 27 21 21
42 42 40 36 36
21 21 23 27 30
69 75 76 78 78 78
89 87 85 82 82
58 54 52 53 52 49 46 48
98 96 95 93 96 92
45 44 42 40 33 28
8 8 11 10 9 11
82 77 76 73 66 63 58
33 35 35 32 29 28 26 27
50 47 45 45 43 40 38 34
66 68 65 62 65 67
13 8 6 5 1 2
10 10 13 16 18 20 22 22
76 77 80 82 80 80
28 24 21 24 22 16
68 64 61 60 59 56 53 55
26 27 26 27 30 33 35 40
61 67 67 68 71 78
34 41 42 45 48 52 54 54
71 74 75 73 74 78
62 62 61 60 57
41 35 33 30 29 27 25 19
41 42 45 46 47 51 48
25 25 28 31 33 31 35
58 62 68 71 74 76 79 82
67 73 76 80 82 84 86
40 40 37 36 38 36
88 84 81 76 73 70 70
35 31 30 29 25
42 35 32 28 23
24 28 25 28 31 29
25 29 32 36 41
5 10 13 15 20 24
9 9 11 12 13 15 18 15
50 46 43 41 40 36 34
40 38 38 39 40 44
24 24 27 28 30 34 36 40
46 48 50 47 45 42 37
99 95 92 90 91 89 85
3 9 10 12 14 18
39 33 31 32 33
73 70 72 76 77 80 80
18 16 19 22 24 30
27 27 24 22 19 16 12 5
29 22 19 16 13 10 4 4
14 16 13 9 4
68 64 62 61 59 57 55 52
22 26 28 29 29
70 74 74 75 77 80 80
85 85 82 80 79 77 75 75
32 36 39 38 40 43 48
54 55 55 57 60 62 62
68 68 71 71 74 76
24 28 31 32 33
85 82 83 85 88 90 94
80 82 87 89 90 89
64 61 59 57 52 51 47
64 70 73 75 72
40 44 46 51 50
48 53 56 58 55 56 56
94 92 89 88 85 87 81
24 19 16 13 12 9 7 7
1 2 4 4 9
94 93 92 95 93 91 89
74 68 68 65 62 59 52
4 7 5 7 5
27 23 20 16 16
56 54 56 59 59
65 65 65 67 68 70 72 72
66 68 73 74 74
62 62 64 66 68
65 69 66 69 71 72 72
83 90 89 90 94
56 56 57 57 59 61 68
93 90 86 85 82 77
34 30 30 29 26 26
34 39 43 45 47 54
12 13 12 10 7 1
59 63 65 64 68
14 14 16 17 21
6 3 4 11 12 19
76 74 71 70 67 67 66 68
50 56 58 60 63 67 71
55 49 46 45 44 43 42 44
78 73 70 68 65 63 59 56
89 86 89 89 91 93 91
65 66 68 69 76 79 81 82
22 21 18 17 16 14 7 4
20 20 23 24 29 31 31
45 50 53 59 61 62 64 71
61 61 60 57 54 53 55
73 69 66 64 63 60 60
60 60 57 56 55 55 57
78 76 76 79 80 83 90
4 4 5 7 13
73 73 73 71 69 69
50 45 44 40 37 33
3 4 7 10 13 16 19 23
86 83 79 76 72
85 84 85 87 90 97 99
17 16 19 22 23 27 28
87 81 78 81 78 76 75 72
22 26 28 30 32 36
38 33 30 29 27
54 56 53 51 47 44 41 38
43 43 44 47 48 52 59
95 92 93 94 95 97
46 46 49 53 54 57 59 57
34 36 36 34 34
39 36 34 34 33
62 58 56 55 57 57
80 83 81 81 79 77 72
34 38 44 47 48 50 53 59
24 19 17 15 15 13
13 17 18 22 23
10 8 9 12 16 18 17
22 18 17 17 10
75 80 79 80 83 84 86 84
59 55 54 53 49 45
34 39 45 46 46
26 22 21 20 13 10 8 9
16 15 13 11 10 9 10
11 13 10 6 4 4
59 61 59 56 53 49 45
57 59 58 51 48 48
33 29 27 26 22 17
85 87 86 80 81
56 56 59 65 67 70 69
86 84 87 93 96 98 96
54 57 60 62 65
52 54 55 58 59 61 63
73 72 71 68 67
45 47 50 51 53 54 56
24 23 22 20 19
73 70 69 68 66 64
30 32 33 35 37 39 40
7 9 10 13 14 17 18 20
67 66 64 61 59
31 28 26 25 24 22 20 18
20 17 14 13 11
18 21 23 26 29 32 35
53 54 57 60 62
68 71 72 74 77 78 81 84
22 24 25 26 29 31 34
15 18 20 21 24 26 29 32
41 39 38 35 34 31
63 65 68 70 72 74 75 76
97 94 92 89 86 84 81 80
71 72 74 77 80 81
54 51 49 48 47 45 42
68 67 66 65 62
24 26 29 30 31 32 34
25 26 29 32 35 36 37
88 86 83 80 77 76 75 74
62 60 57 54 52 50 49
64 62 59 58 56
54 55 57 60 63 66
28 30 31 32 34
63 62 60 57 56 54 51 49
83 85 86 88 89 91 92
1 4 5 7 10 13 15
32 31 30 28 26 24 23 20
84 86 88 90 92 94 95
24 23 21 20 19 18
92 91 88 85 84
23 26 27 28 31 32 33 36
36 38 39 40 43 45 47 49
29 26 24 21 20 17
12 10 9 7 4 3 2
25 23 21 19 17 15 13 12
38 37 35 32 31
11 12 15 18 19
99 96 95 93 90
39 37 36 33 30
35 37 39 40 43
79 76 74 73 70 68
40 37 34 33 30 28
71 70 68 67 65 64 63 62
95 92 91 89 86 83
45 44 43 40 39 36
29 26 25 23 22 21
67 68 71 73 75 78 79 82
95 92 89 88 86 85
6 7 10 11 12 14
46 45 43 41 38 36
31 29 27 26 25 22
47 50 53 55 57
70 71 73 76 77
13 16 17 19 22 25 26 28
40 42 43 44 47 50 51 52
40 41 42 45 47 50
70 71 74 76 78
65 68 71 73 76
23 26 29 30 33
48 51 53 56 57 58
69 70 73 74 77 78 80 81
28 30 32 35 36
67 68 69 71 73 76
93 90 87 85 82 80
67 65 64 62 60
38 35 33 31 30 27 24 23
98 96 94 92 89 86
14 11 9 8 7 4
71 72 73 76 77 79 82 83
80 79 78 75 73 72 69
10 8 6 5 3
85 84 82 79 76 74 73 72
60 57 56 55 54
67 66 64 61 58 57 55
56 59 62 63 65 66 69 70
49 52 55 56 57 58 59 61
98 96 94 92 89 86 83 81
62 64 65 68 69
33 35 36 37 38 41 43
76 79 81 84 86
22 23 26 29 32 34 36
38 36 33 31 28
77 75 74 73 72 69 67
49 52 55 56 57 60 62
92 91 88 86 85 83 80
60 58 57 56 53 51 48
34 37 40 43 44 47 49
19 18 17 15 12 11
86 83 81 79 76 75
63 61 60 59 56
62 63 66 68 70 73 74 77
53 56 58 60 63 64 65 67
4 6 8 9 10
73 76 78 80 81
66 64 61 59 58 57 54
82 80 79 77 75
14 15 18 21 23
10 11 14 17 19
32 34 37 40 43
67 68 71 74 76 78
85 83 80 79 77 76 73
24 21 19 17 15 14 11
24 23 20 17 15 13 11
55 58 59 60 63 65 66 68
37 36 33 30 28
56 53 52 49 47 46 43
76 73 71 69 67 64 63
86 83 82 81 79 76 74
74 75 77 80 82 83 85 87
65 68 70 71 72 75 77
51 48 45 42 39
45 43 41 39 37 34
14 16 19 21 23 25
39 41 44 45 48 50 53
51 50 49 46 45 43 42 39
34 31 28 26 25 23 22
55 57 60 62 64 66 67
66 64 63 61 59 56 53
33 35 38 41 42 44 46
38 36 33 32 31 30 27
26 27 30 31 32 35
24 25 26 29 32 33
65 64 63 61 58
22 25 27 30 32 35 37 38
33 32 30 27 25
71 74 75 76 77
82 81 80 79 78 76 75 74
49 48 46 44 43 41 39
41 42 44 46 47
81 83 86 89 92 93
12 14 16 18 20 23
89 86 84 82 79 76 75
75 76 77 78 79 81
79 77 74 73 70 68 65 62
12 14 17 18 19 20
6 7 9 10 13 16 19 22
66 65 64 62 61
61 60 57 56 54 51 48 47
69 67 65 63 62 60 58
23 21 19 18 17
20 23 25 27 28
29 28 27 24 23 21 18
68 66 65 64 63 61
64 61 58 55 52 50 47
70 67 64 61 60 57
37 35 32 31 28 27 25
59 60 62 65 68 69 72 75
46 44 43 42 41 38 35
19 21 24 26 27 28 31
27 28 29 31 34 36 37 39
80 78 75 73 70 69 68 65
36 33 31 29 28
88 85 82 79 77 74
9 12 15 18 19
16 14 11 10 8 5 4 3
97 95 94 92 91 89
47 48 50 52 54 55 58 61
44 47 48 49 50 51
50 51 53 56 58
53 54 55 57 60 63 66
18 21 22 23 25 27
93 90 87 84 82 80 79
60 63 64 66 69 72 75 77
94 92 90 87 85 84 81
37 39 42 45 48
63 66 69 70 71 72
46 47 50 52 54 57 60
2 4 7 10 12
41 38 37 34 33 31
18 21 23 25 28 31
16 15 14 12 11 10 7 5
30 32 34 37 40 42 45 46
1 3 4 6 9 10
87 84 83 81 80 77
49 51 54 55 57 58 60 62
19 18 16 14 11 9
1 2 3 4 5 8
68 70 72 73 74
35 32 31 29 26 23 22 20
51 53 56 59 62 63 65 67
79 81 84 85 87 90 93
51 54 55 56 57 58
21 23 24 25 28 29
35 34 31 29 28 27 26
30 27 25 23 20 17 16 14
30 28 26 24 22 21 20 17
18 21 22 25 28 31 32 34
81 78 77 75 74
72 75 78 81 83
84 85 88 89 92 94 95 96
15 17 18 20 21 23
80 78 76 74 73 70 68 65
54 51 50 48 46 45
85 84 83 80 79 78 77
34 31 30 28 26 23
69 72 75 76 77 78 81
52 54 55 57 59
82 79 76 73 71 68 65 62
72 74 75 76 79 80
12 9 7 6 4 3 2 1
50 51 52 53 55 58 59 60
20 22 25 28 29
67 70 73 75 76
22 25 26 29 30
48 50 53 56 57 59 62 63
70 71 72 75 77 80 82
87 84 81 79 77
93 92 90 89 87
69 70 71 73 74 77
11 14 17 19 21 24 25 28
85 88 89 92 94
61 64 65 68 69 71 72
6 7 9 12 14
49 52 53 54 57 58 59
57 54 51 49 48 47 45
66 63 61 58 57 54 53 52
25 24 22 19 16 15 12 11
43 41 40 37 34 33 32
93 91 88 86 84 82 81 78
40 43 46 47 49 50
56 53 51 48 47
42 45 48 50 52 55
62 60 57 54 52 51
51 48 45 44 43 42 40
28 26 23 22 19 16 13 11
67 64 62 61 58 57
80 83 84 87 88 89
51 52 55 57 60 63 66
13 10 7 5 3
68 71 73 76 79 81 82
91 89 86 85 82 81 80
31 29 28 27 24 22 21
21 18 17 14 11 10 9 8
72 75 78 80 83 85
55 57 58 61 63 66 69
50 48 47 46 45
76 79 82 85 87 88
49 48 46 44 42 40 39 38
68 69 71 74 76
22 21 19 16 13
27 24 23 22 19 18
19 18 16 14 13 12 11
79 81 84 86 89
4 6 7 8 9 10
20 23 26 28 29 30
84 87 88 90 92
31 33 34 37 39 41 43
31 28 27 25 23 20
49 52 55 58 60 62 65
75 77 78 81 83 84 85 86
56 53 51 50 47 44 42 39
26 29 30 33 35 36 37 40
63 66 67 69 72 73 74
6 7 10 11 13 14 16
67 66 63 62 61 60 59 58
60 61 63 64 66 68 70
19 17 16 15 13 12 9 6
88 85 82 79 77 76 74
15 14 12 11 9
3 4 5 8 10 12
14 12 9 7 6 4
40 37 36 34 33 31
28 31 34 36 37 39 40
30 32 33 35 36 37
62 60 58 57 55 54 51
60 58 55 52 50
57 58 61 63 66 69 70 72
81 80 77 76 75 73
72 70 69 66 64 62 59
84 81 78 77 75 74 73
79 82 84 87 89 92
84 87 88 90 92 94
28 25 23 22 21
57 56 53 52 51 50 48
25 27 29 31 32 35 36
20 22 25 27 28 31
41 40 37 34 32 30 28
52 55 58 59 60 62 65
57 55 54 52 50 48 45
81 78 75 74 71
73 76 78 80 82
26 25 22 20 19 16 15 13
68 67 65 62 60 58 56
43 41 39 37 35
70 73 74 77 79 80
88 85 83 80 77 74 72
82 79 76 73 70
56 53 52 49 46 43 40
45 42 40 39 38 35 32
21 23 24 27 30 31 32 35
28 30 31 32 33 34 35 37
66 69 71 73 74 75
56 57 60 61 64
81 79 77 76 73 71
81 79 78 75 72 70
55 56 57 59 62 65 67
55 52 50 48 45
9 7 4 3 2 1
27 28 31 33 35 36
89 88 86 85 83 82 81 78
4 7 10 11 12 14 16 19
49 51 53 56 57 59 62
30 29 26 23 21 18 16 15
67 66 64 61 60
62 61 59 56 55
57 59 62 64 67
71 72 75 77 78 79 81
45 47 50 52 54 56
21 23 26 28 30 33 36 38
54 55 58 61 63 65 67 70
50 49 48 47 45 43
70 71 72 73 75 77
22 24 27 30 32 33 36 37
83 86 89 91 93 94 97
32 31 29 27 24
68 69 72 75 78 80
64 65 66 67 70 73
37 35 32 29 26
39 37 34 33 30 27
15 16 17 18 20
75 73 72 70 69 68 65 63
45 47 48 51 54 56
29 28 27 25 22 19 18 15
4 7 8 9 12 13 16 17
62 65 66 69 70 72
26 24 21 18 15 13 10 7
2 5 6 9 12 13
70 69 67 66 65 62
37 40 42 45 48
92 90 89 88 86 83 80 79
72 70 69 66 63
35 33 31 30 29 28
56 58 60 61 63 65
58 56 55 52 50
35 32 29 28 27 24 21
99 96 95 92 89
15 16 19 21 22 23 25
1 3 4 7 10 13 14 17
89 88 87 85 84 82 81 78
29 30 32 33 36 37
73 75 78 79 80 82 84
67 69 72 75 77
45 42 41 38 37 35 34
40 43 46 48 49
54 56 58 59 60 63 64 66
24 26 29 32 33
95 93 90 87 84
71 68 65 63 62 61
53 51 50 48 47
32 30 28 27 24 23
46 49 52 53 55 57 58
54 57 60 62 63 65 67
12 13 14 17 20 22 25 28
36 38 40 42 45 46 48
84 85 86 87 89
64 62 59 57 56 53 50 48
54 57 59 62 65 66 69 70
82 81 80 79 77 75
21 22 24 26 27 30 33
52 49 46 45 44 41
47 49 51 52 54 56 59 60
67 68 70 71 73 75
13 15 17 19 21 22 23
25 27 28 31 34 36 38 39
23 20 17 16 13 10 9 8
78 80 82 85 86 89 91 94
82 85 87 89 90 93 94
51 48 47 44 41 38 35
17 20 23 25 27
19 22 23 25 28 31 34
76 74 73 71 68
49 52 53 55 57 58 59 61
52 54 56 59 62
73 70 68 66 64 62 61 60
92 89 88 87 85
21 24 27 28 29
23 20 19 16 14
44 45 48 51 52
44 41 39 38 35
42 41 38 35 32 31
17 18 19 20 21 23
27 30 32 34 36 38 41 44
9 11 12 13 16 17
16 13 10 8 5 4 2
52 53 56 58 59 60 61
29 32 34 36 39 41 44
77 78 81 84 87
61 59 56 53 51 49 48 45
52 53 55 56 58 60 63
24 25 28 31 32 35 38 40
20 22 25 27 30
65 67 68 71 74 75
45 48 50 51 53 54 56
70 69 66 64 63 62
30 29 27 26 24 22
53 55 58 59 61 62
31 29 27 26 24 22 20 18
2 4 7 9 12 13
39 38 37 35 34 31 29
60 58 56 53 51 49 48
4 7 8 10 12 14
40 38 35 32 29 28 26 24
90 88 86 83 82 79
56 57 58 61 64 66 69 72
80 82 83 85 87 89 90
39 38 36 33 30 29
33 30 29 27 24 22 21 18
54 57 58 59 60 62
81 84 86 87 90 92 93 96
22 23 26 28 31
70 68 67 66 63
29 27 25 23 22 20
25 27 28 31 32 35 38 39
77 76 73 70 68 67
61 59 58 57 56
40 41 43 46 48
31 28 26 23 22 21 20 17
45 43 41 38 35
46 48 49 51 54 55
50 52 53 54 55 58 61 64
16 13 11 10 9 7 4 1
34 37 38 40 42
97 95 94 93 90 89 88 87
30 32 34 36 38 39 41 43
73 70 67 66 64 63 62
82 80 79 76 73 72
53 50 47 45 44 41 40 39
81 83 86 88 91 93 94
24 22 20 17 16 15
55 56 58 59 60 63 65 67
9 12 14 17 18 19 22
19 16 15 13 10 7 5
97 95 94 92 91 89 88 87
78 80 82 83 85 86 88 90
45 48 51 52 53 54
54 53 52 50 48 47 46 45
55 54 53 52 50
38 35 34 33 30 27 26
65 62 61 59 58 55
33 35 36 38 41 42 43 45
35 32 29 28 27 26 23
39 37 35 32 31 28 26 24
2 3 4 6 7 10 12 15
17 19 22 23 25 28
53 55 58 60 63 65 68
64 67 70 72 75
78 75 72 70 68 67
56 54 51 50 47 44
21 18 15 12 11 9
56 54 51 49 48 45 42 39
50 47 44 42 41 39 37
72 75 77 78 81 83 84
85 84 83 82 79 76
83 81 78 75 72
80 82 84 87 90 93 95
81 83 84 87 89 90
68 71 72 74 77 78 79
70 71 72 75 78 81 83 84
20 19 16 15 13
12 9 8 7 6 5 2
22 23 25 26 28 29 31 32
32 30 28 25 22 20
35 32 31 28 25 23
36 39 40 41 44 46 48 50
52 55 56 59 62
50 48 45 43 40 39 37 36
36 35 33 32 31 28
50 48 47 44 43 41 38 36
6 5 4 2 1
19 16 14 12 9 7 4
28 30 32 33 35
75 73 70 68 65
25 23 20 18 17 14 12
67 70 72 73 76 77 78
73 75 76 79 82 83
37 35 34 31 29
99 97 95 93 91 89
8 9 10 11 12 15
60 59 56 55 52 49 48 47
54 53 51 50 48 47
32 35 37 39 40 42
79 78 77 76 75
43 42 40 37 34 31
22 23 25 28 31
98 96 93 92 89 88 85
60 59 56 54 52 50 48 47
71 72 73 74 77 78 81
37 34 32 30 28 27
18 21 22 23 24
29 31 33 35 36 39 42 44
36 33 30 29 26 23 21 18
44 46 47 49 50 53 55 57
69 66 65 62 59
80 81 83 84 87
62 65 68 71 74
56 53 52 50 47
36 39 42 43 45 47 49
68 65 64 62 60 57 54
78 75 73 71 69 67 65 63
50 52 54 55 57 59 60
73 72 71 70 69 67
99 96 93 90 87
34 32 31 28 27 24 22
78 81 82 84 85 87 88
37 35 34 33 30
70 69 68 65 63 62
85 82 80 77 75 74 72 69
33 35 38 41 42
67 64 62 60 58
38 36 35 34 31 30
64 67 69 71 74 77
89 86 83 80 79 77
80 77 75 74 71
76 79 82 83 86
92 89 86 85 84 82 79 77
14 17 20 22 25 28
23 26 27 29 32 35 36
99 98 96 93 90 87 84
41 43 44 47 49 52 53 56
25 24 21 19 16 13 11
46 49 52 54 56
68 65 62 61 58
73 70 68 65 62 61 60
42 43 45 48 49
61 64 67 70 72 74
71 72 73 74 75 76
89 86 84 81 79 76 73 72
48 46 45 42 39 36 34 32
66 63 61 59 56 55 53
18 20 22 24 27 30 33
50 49 47 46 44
69 70 71 74 75 78 79
18 16 13 12 11 8 7
57 59 60 62 64 67
25 27 29 32 35
71 68 67 65 64 61 58
21 22 24 26 29 32 34 37
89 87 85 82 79 78 75
91 89 87 85 82 81 78
77 79 81 82 85 86
59 61 62 65 67 68
94 93 92 91 90 87 85 83
50 52 53 54 56 57 58 61
"
}