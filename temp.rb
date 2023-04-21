N = gets.to_i
A = gets.split.map(&:to_i)

array = []

(1...N).each do |i|
  if A[i-1] <= A[i] == A[i] <= A[i+1]
    next
  else
    array.push(i)
  end
end

puts array.count + 1
