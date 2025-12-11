#!/usr/bin/awk -f

function pow2(n,    r,i){
    r = 1
    for (i = 1; i <= n; i++) r *= 2
    return r
}

function bxor(a, b,    r, bit, abit, bbit) {
    r   = 0
    bit = 1
    while (a > 0 || b > 0) {
        abit = a % 2
        bbit = b % 2
        if (abit != bbit)
            r += bit
        a   = int(a / 2)
        b   = int(b / 2)
        bit *= 2
    }
    return r
}

function mask_from_group(g,    inner,arr,n,m,i) {
    inner = substr(g,2,length(g)-2)
    if (inner == "") return 0
    n = split(inner,arr,",")
    m = 0
    for (i = 1; i <= n; i++) {
        if (arr[i] != "")
            m += pow2(arr[i])
    }
    return m
}

function gf2_rank(rows, num_rows, num_cols,
                  r, c, i, pivot_row, bit, limit, tmp) {

    r = 0

    for (c = 0; c < num_cols; c++) {
        bit   = pow2(c)
        limit = pow2(c+1)

        pivot_row = -1
        for (i = r; i < num_rows; i++) {
            if ((rows[i] % limit) >= bit) {
                pivot_row = i
                break
            }
        }

        if (pivot_row < 0)
            continue

        tmp = rows[r]
        rows[r] = rows[pivot_row]
        rows[pivot_row] = tmp

        for (i = 0; i < num_rows; i++) {
            if (i != r && (rows[i] % limit) >= bit)
                rows[i] = bxor(rows[i], rows[r])
        }

        r++
        if (r == num_rows)
            break
    }

    return r
}

{
    print $0

    delete rows
    delete button_mask

    num_buttons  = 0
    num_counters = 0
    max_b        = 0

    # parse one machine line
    for (i = 1; i <= NF; i++) {
        t = $i
        if (t ~ /^\[/) continue

        if (t ~ /^\(/) {
            button_mask[num_buttons++] = mask_from_group(t)
            continue
        }

        if (t ~ /^\{/) {
            inner = substr(t,2,length(t)-2)
            n = split(inner,arr,",")
            num_counters = n
            for (j = 1; j <= n; j++) {
                val = arr[j] + 0
                if (val > max_b) max_b = val
            }
            break
        }
    }

    # build GF(2) matrix rows[c] = bitmask over buttons
    # rows[c] represents: which buttons affect counter c
    for (c = 0; c < num_counters; c++)
        rows[c] = 0

    for (b = 0; b < num_buttons; b++) {
        m    = button_mask[b]
        bitb = pow2(b)
        for (c = 0; c < num_counters; c++) {
            bitc = pow2(c)
            if ((m % (2 * bitc)) >= bitc)
                rows[c] += bitb
        }
    }

    # Debug: print the matrix
    print "Matrix (counters × buttons):"
    for (c = 0; c < num_counters; c++) {
        printf "Counter %2d: ", c
        for (b = 0; b < num_buttons; b++) {
            bitb = pow2(b)
            if ((rows[c] % (2*bitb)) >= bitb)
                printf "1"
            else
                printf "0"
        }
        printf "  (mask=%d)\n", rows[c]
    }

    rank = gf2_rank(rows, num_counters, num_buttons)
    free = num_buttons - rank

    cost = 1
    for (k = 1; k <= free; k++)
        cost *= (max_b + 1)

    print "rank = " rank ", free = " free ", cost = " cost "\n"
}