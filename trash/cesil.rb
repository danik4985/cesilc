
instructions = []
data = []
dc = 0
pc = 0
a = 0
literals = {}

while gets
	if $_ =~ /^\s+%\s+$/
		while gets
			data << $_.strip
		end

	elsif $_ =~ /^(([A-Z][A-Z0-9]*)\s+|\s+)([A-Z]+)\s+(.*)$/
		label, operation, operand = $1, $3, $4
		
		instructions << { :label => label.strip, :operation => operation.strip, :operand => operand.strip }

	elsif $_.strip.size > 0
		raise "Syntax error."
	end
end


while true
	instruction = instructions[ pc ]
		
	case instruction[ :operation ]
	when "LOAD"
		if instruction[ :operand ] =~ /^[0-9]+$/
			a = instruction[ :operand ].to_i
		else
			if literals.keys.include? instruction[ :operand ]
				a = literals[ instruction[ :operand ] ]
			else
				raise "#{instruction[ :operand ]} doesn't exist as a literal in line #{pc}."
			end
		end

		pc += 1

	when "STORE"
		literals[ instruction[ :operand ] ] = a

		pc += 1

	when "IN"
		if dc > data.size
			raise "You've read too much data in line #{pc}."
		end
		
		a = data[ dc ].to_i

		dc += 1
		pc += 1

	when "ADD"
		if not literals.keys.include? instruction[ :operand ]
			raise "Literal #{instruction[ :operand ]} doesn't exist in line #{pc}."
		end

		a += literals[ instruction[ :operand ] ]
		
		pc += 1

	when "SUBTRACT"
		if not literals.keys.include? instruction[ :operand ]
			raise "Literal #{instruction[ :operand ]} doesn't exist in line #{pc}."
		end
		
		a -= literals[ instruction[ :operand ] ]

		pc += 1

	when "MULTIPLY"
		if not literals.keys.include? instruction[ :operand ]
			raise "Literal #{instruction[ :operand ]} doesn't exist in line #{pc}."
		end
		
		a *= literals[ instruction[ :operand ] ]

		pc += 1

	when "DIVIDE"
		if not literals.keys.include? instruction[ :operand ]
			raise "Literal #{instruction[ :operand ]} doesn't exist in line #{pc}."
		end
		
		a /= literals[ instruction[ :operand ] ]

		pc += 1

	when "JUMP"
		found = false
		instructions.each_with_index do |label_hunt, c|
			if label_hunt[ :label ] == instruction[ :operand ]
				found = true
				pc = c
			end
		end
		
		if found == false
			raise "Couldn't find label #{instruction[ :operand ]} in line #{pc}."
		end

	when "JIZERO"
		if a == 0
			found = false
			instructions.each_with_index do |label_hunt, c|
				if label_hunt[ :label ] == instruction[ :operand ]
					found = true
					pc = c
				end
			end
			
			if found == false
				raise "Couldn't find label #{instruction[ :operand ]} in line #{pc}."
			end

		else
			pc += 1
		end

	when "JINEG"
		if a < 0
			found = false
			instructions.each_with_index do |label_hunt, c|
				if label_hunt[ :label ] == instruction[ :operand ]
					found = true
					pc = c
				end
			end
			
			if found == false
				raise "Couldn't find label #{instruction[ :operand ]} in line #{pc}."
			end

		else
			pc += 1
		end
		
	when "PRINT"
		print instruction[ :operand ].gsub( /^\"/, "" ).gsub( /\"$/, "" )
		pc += 1

	when "OUT"
		print a
		pc += 1

	when "LINE"
		puts
		pc += 1

	when "HALT"
		exit
		
	else
		raise "Syntax error in line #{pc}."

	end
end