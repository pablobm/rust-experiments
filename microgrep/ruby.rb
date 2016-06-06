ARGV.each do |path|
  file = File.open(path)
  file.each_line do |line|
    if line =~ %r{\d\d\.\dms\)}
      puts line
    end
  end
end
