#!/usr/bin/awk -f

function mask_from_group(group,    inner, arr, n, m, i) {
    inner = substr(group, 2, length(group)-2)
    if (inner == "") return 0
    n = split(inner, arr, ",")
    m = 0
    for (i = 1; i <= n; i++)
        m += (2 ^ arr[i])
    return m
}

function gf2_rank(rows, num_rows, num_buttons,
                  r, c, i, pivot_row, bit) {
    r = 0
    for (c = 0; c < num_buttons; c++) {
        bit = (2 ^ c)
        pivot_row = -1

        for (i = r; i < num_rows; i++) {
            if ((rows[i] % (2^(c+1))) >= bit) { pivot_row = i; break }
        }
        if (pivot_row < 0) continue

        tmp = rows[r]; rows[r] = rows[pivot_row]; rows[pivot_row] = tmp

        for (i = 0; i < num_rows; i++) {
            if (i != r && (rows[i] % (2^(c+1))) >= bit)
                rows[i] = rows[i] ^ rows[r]
        }

        r++
        if (r == num_rows) break
    }
    return r
}

{
    delete button_mask
    delete rows
    delete jolts

    num_buttons = 0
    num_counters = 0
    max_b = 0

    for (i = 1; i <= NF; i++) {
        tok = $i

        if (tok ~ /^\[/) continue

        if (tok ~ /^\(/) {
            button_mask[num_buttons] = mask_from_group(tok)
            num_buttons++
            continue
        }

        if (tok ~ /^\{/) {
            inner = substr(tok, 2, length(tok)-2)
            if (inner != "") {
                n = split(inner, arr, ",")
                num_counters = n
                for (j = 1; j <= n; j++) {
                    jolts[j-1] = arr[j] + 0
                    if (arr[j]+0 > max_b) max_b = arr[j]+0
                }
            }
        }
    }

    # Build rows
    for (r = 0; r < num_counters; r++)
        rows[r] = 0

    for (b = 0; b < num_buttons; b++) {
        mask = button_mask[b]
        for (c = 0; c < num_counters; c++) {
            bitc = (2 ^ c)
            if ((mask % (2^(c+1))) >= bitc)
                rows[c] += (2 ^ b)
        }
    }

    rank = gf2_rank(rows, num_counters, num_buttons)
    free = num_buttons - rank

    # cost = (max_b+1)^free
    cost = 1
    for (k = 1; k <= free; k++)
        cost *= (max_b + 1)

    # log10 bin
    if (cost < 1) bin = 0
    else bin = int(log(cost)/log(10))

    hist[bin]++
    max_bin = (bin > max_bin ? bin : max_bin)
}

END {
    print "Histogram of search-space cost (log10 bins):"
    for (b = 0; b <= max_bin; b++) {
        low = (b == 0 ? 0 : sprintf("1e%d", b))
        high = sprintf("1e%d", b+1)
        printf("bin %-2d [%s .. %s): %d lines\n", b, low, high, (b in hist ? hist[b] : 0))
    }
}
