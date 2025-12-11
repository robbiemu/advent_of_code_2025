#!/bin/bash

# Save stdin to a temp file so we can process it multiple times
tmpfile=$(mktemp)
cat > "$tmpfile"

# Extract and sort costs (also clean commas from cost!)
echo "=== Top 20 Most Expensive Cases ==="
awk '{
    rank = $3; gsub(/,/, "", rank)
    free = $6; gsub(/,/, "", free)
    cost = $9; gsub(/,/, "", cost)
    print cost, rank, free
}' "$tmpfile" | sort -k1,1nr | head -20 | awk '{printf "cost = %12s, rank = %2s, free = %s\n", $1, $2, $3}'

echo ""
echo "=== Cost Distribution ==="
awk '{
    cost = $9; gsub(/,/, "", cost)
    if (cost == 1) bucket = "cost = 1"
    else if (cost < 100) bucket = "1 < cost < 100"
    else if (cost < 1000) bucket = "100 <= cost < 1K"
    else if (cost < 10000) bucket = "1K <= cost < 10K"
    else if (cost < 100000) bucket = "10K <= cost < 100K"
    else if (cost < 1000000) bucket = "100K <= cost < 1M"
    else if (cost < 10000000) bucket = "1M <= cost < 10M"
    else if (cost < 100000000) bucket = "10M <= cost < 100M"
    else bucket = "cost >= 100M"
    count[bucket]++
}
END {
    for (b in count) print count[b], b
}' "$tmpfile" | sort -rn

echo ""
echo "=== Free Variable Distribution ==="
awk '{
    free = $6; gsub(/,/, "", free)
    count[free]++
}
END {
    for (f in count) printf "free = %s: %3d cases\n", f, count[f]
}' "$tmpfile" | sort -n

echo ""
echo "=== Statistics ==="

# Extract and clean all costs
costs_clean=$(awk '{
    cost = $9; gsub(/,/, "", cost)
    if (cost ~ /^[0-9]+$/) print cost
}' "$tmpfile")

# Count total lines
total_cases=$(echo "$costs_clean" | wc -l)

if [ "$total_cases" -eq 0 ] || [ -z "$(echo "$costs_clean" | head -n1)" ]; then
    echo "Total cases: 0"
    echo "Max cost: 0"
    echo "Average cost: 0"
    echo "Median cost: 0"
    rm -f "$tmpfile"
    exit 0
fi

# Compute sum, max, average
stats=$(echo "$costs_clean" | awk '{
    sum += $1
    if (NR == 1 || $1 > max) max = $1
}
END {
    printf "%d %.0f %d", NR, (NR ? sum/NR : 0), max
}')
read total_cases avg_cost max_cost <<< "$stats"

# Compute median using sort + awk (POSIX-safe)
if [ "$total_cases" -gt 0 ]; then
    sorted_costs=$(echo "$costs_clean" | sort -n)
    if [ $((total_cases % 2)) -eq 1 ]; then
        # Odd: pick middle line
        median=$(echo "$sorted_costs" | sed -n "$(((total_cases + 1) / 2))p")
    else
        # Even: average two middle lines
        mid1=$(echo "$sorted_costs" | sed -n "$((total_cases / 2))p")
        mid2=$(echo "$sorted_costs" | sed -n "$(((total_cases / 2) + 1))p")
        median=$(( (mid1 + mid2) / 2 ))
    fi
else
    median=0
fi

printf "Total cases: %d\n" "$total_cases"
printf "Max cost: %d\n" "$max_cost"
printf "Average cost: %.0f\n" "$avg_cost"
printf "Median cost: %d\n" "$median"

rm -f "$tmpfile"