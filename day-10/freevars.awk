function bw_xor(a, b,    r, bit) {
    r = 0
    bit = 1
    while (a > 0 || b > 0) {
        if ((a % 2) != (b % 2))
            r += bit
        a = int(a / 2)
        b = int(b / 2)
        bit *= 2
    }
    return r
}

function mask_from_group(g,    inner, arr, n, i, m) {
    inner = substr(g, 2, length(g)-2)
    if (inner == "") return 0
    n = split(inner, arr, ",")
    m = 0
    for (i = 1; i <= n; i++)
        m += (2 ^ arr[i])
    return m
}

function gf2_rank(rows, nr, nc,    r, c, i, piv, bit, tmp) {
    r = 0
    for (c = 0; c < nc; c++) {
        bit = (2 ^ c)

        piv = -1
        for (i = r; i < nr; i++) {
            if ((rows[i] % (2^(c+1))) >= bit) {
                piv = i
                break
            }
        }
        if (piv < 0) continue

        tmp = rows[r]; rows[r] = rows[piv]; rows[piv] = tmp

        for (i = 0; i < nr; i++) {
            if (i != r && (rows[i] % (2^(c+1))) >= bit)
                rows[i] = bw_xor(rows[i], rows[r])
        }

        r++
        if (r == nr) break
    }
    return r
}

{
    delete rows
    delete btn

    num_buttons = 0
    num_lights  = 0

    # Parse pattern + buttons
    for (i = 1; i <= NF; i++) {
        tok = $i

        # lights: [.##.#...] – just need length
        if (tok ~ /^\[/) {
            inner = substr(tok, 2, length(tok)-2)
            num_lights = length(inner)
            continue
        }

        # buttons: (0,1,3,4)
        if (tok ~ /^\(/) {
            btn[num_buttons] = mask_from_group(tok)
            num_buttons++
            continue
        }

        # ignore {jolts...}
        if (tok ~ /^\{/)
            break
    }

    # Build rows: one per light
    for (r = 0; r < num_lights; r++)
        rows[r] = 0

    for (b = 0; b < num_buttons; b++) {
        m = btn[b]
        for (l = 0; l < num_lights; l++) {
            bit = (2 ^ l)
            if ((m % (2^(l+1))) >= bit)
                rows[l] += (2 ^ b)
        }
    }

    rank = gf2_rank(rows, num_lights, num_buttons)
    free = num_buttons - rank
    hist[free]++
}

END {
    print "Histogram of FREE VARIABLES:"
    for (f = 0; f <= 20; f++)
        if (f in hist)
            printf "free = %2d : %d machines\n", f, hist[f]
}