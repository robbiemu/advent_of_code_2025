#!/usr/bin/awk -f

function pow2(n, r,i){
    r = 1
    for (i = 1; i <= n; i++) r *= 2
    return r
}

function bxor(a, b, r, bit, abit, bbit) {
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

function mask_from_group(g, inner,arr,n,m,i) {
    inner = substr(g,2,length(g)-2)
    if (inner == "") return 0
    n = split(inner,arr,",")
    m = 0
    for (i = 1; i <= n; i++)
        m += pow2(arr[i]+0)
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

        if (pivot_row < 0) continue

        tmp = rows[r]; rows[r] = rows[pivot_row]; rows[pivot_row] = tmp

        for (i = 0; i < num_rows; i++) {
            if (i != r && (rows[i] % limit) >= bit)
                rows[i] = bxor(rows[i], rows[r])
        }

        r++
        if (r == num_rows) break
    }
    return r
}

# --------------------------
# MAIN
# --------------------------
{
    delete button_mask
    delete rows

    num_buttons = 0
    num_lights  = 0

    for (i = 1; i <= NF; i++) {
        t = $i

        if (t ~ /^\[/) {
            inner = substr(t,2,length(t)-2)
            num_lights = length(inner)
            for (j = 0; j < num_lights; j++)
                rows[j] = 0
            continue
        }

        if (t ~ /^\{/) break

        if (t ~ /^\(/)
            button_mask[num_buttons++] = mask_from_group(t)
    }

    # rows[light] = bitmask of buttons affecting that light
    for (b = 0; b < num_buttons; b++) {
        m = button_mask[b]
        bitb = pow2(b)

        for (light = 0; light < num_lights; light++) {
            bitc = pow2(light)
            if ((m % pow2(light+1)) >= bitc)
                rows[light] += bitb
        }
    }

    rank = gf2_rank(rows, num_lights, num_buttons)
    free = num_buttons - rank
    naive_cost = pow2(num_buttons)

    # IMPORTANT: match Part 2 format so analyze_costs.sh works:
    # rank = $3, free = $6, cost = $9
    printf "rank = %d, free = %d, cost = %d\n", rank, free, naive_cost
}
