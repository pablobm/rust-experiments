SOLUTIONS = {
  '001' => 233168,
  '002' => 4613732,
  '003' => 6857,
  '004' => 906609,
  '005' => 232792560,
  '006' => 25164150,
  '007' => 104743,
  '008' => 23514624000,
  '009' => 31875000,
  '010' => 142913828922,
}

$successes = 0
$failures = 0

def test_problems
  SOLUTIONS.each do |number, solution|
    result = `cd p#{number} && cargo run`.split.last.to_i
    if result == solution
      $successes += 1
    else
      puts "FAILURE: expected #{solution}, but got #{result}"
      $failures += 1
    end
  end
end

def test_support_library
  output = `cd euler && cargo test`
  if $? == 0
    $successes += 1
  else
    puts output
    $failures += 1
  end
end

test_support_library
test_problems

prefix = $failures == 0 ? "SUCCESS" : "FAILURE"
puts "#{prefix}: #{$successes} successes, #{$failures} failures"
