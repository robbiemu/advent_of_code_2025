#!/usr/bin/awk -f

function pow2(n, r,i){ r=1; for(i=1;i<=n;i++)r*=2; return r }

function mask_from_group(g, inner,arr,n,m,i) {
    inner = substr(g,2,length(g)-2)
    if (inner == "") return 0
    n = split(inner,arr,",")
    m = 0
    for (i = 1; i <= n; i++)
        m += pow2(arr[i]+0)
    return m
}

{
    delete rows
    delete button_mask

    num_buttons = 0
    num_lights = 0

    for (i=1;i<=NF;i++) {
        t=$i
        if (t ~ /^\[/) {
            inner=substr(t,2,length(t)-2)
            num_lights=length(inner)
            for (j=0;j<num_lights;j++) rows[j]=0
            continue
        }
        if (t ~ /^\{/) break
        if (t ~ /^\(/) button_mask[num_buttons++] = mask_from_group(t)
    }

    # Build rows
    for (b=0;b<num_buttons;b++) {
        m = button_mask[b]
        printf("button %d mask = %d\n", b, m)
        for (light=0; light<num_lights; light++) {
            bitc = pow2(light)
            if ((m % pow2(light+1)) >= bitc) {
                rows[light] += pow2(b)
            }
        }
    }

    print "ROWS BEFORE RANK:"
    for (light = 0; light < num_lights; light++)
        printf("row[%d] = %d\n", light, rows[light])
}
