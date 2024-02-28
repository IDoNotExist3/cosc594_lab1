.PHONY: all one two three four

EX1 = $(./happyQR --dark img 100 blackFox.jpg --light img 100 bulldog.jpg --output example1.jgr "question mark?")
COMP1 = $(jgraph -P example1.jgr | ps2pdf - | convert -density 300 - -quality 100 example1.jpg)
EX2 = $(./happyQR --light col 100 250 200 250 --dark pat 50 "linethickness 1 color 0 0 0 pcfill 0 0 0 ppattern stripe 75" --output example2.jgr "question mark?")
COMP2 = $(jgraph -P example2.jgr | ps2pdf - | convert -density 300 - -quality 100 example2.jpg)
all: compile one two three four five
compile:
	cargo build --release
	cp target/release/happyQR .
one: 
	./happyQR --light col 100 250 200 250 --dark col 100 100 0 100 --output example1.jgr "Example 1?"
	jgraph -P example1.jgr | ps2pdf - | convert -density 300 - -quality 100 example1.jpg
two:
	./happyQR --dark col 10 0 100 0 --dark col 10 100 0 0 --dark col 10 0 0 100 --dark col 10 50 50 50 --dark col 10 50 0 50 --dark col 10 50 0 50 --dark col 10 0 50 50 --dark col 10 0 0 50 --dark col 10 0 50 0 --dark col 10 50 0 0 --light col 10 200 200 200 --light col 10 250 150 250 --light col 10 250 250 150 --light col 10 250 200 250 --light col 10 200 250 250 --light col 10 250 250 200 --light col 10 200 200 250 --light col 10 200 250 200 --light col 10 250 200 200 --output example2.jgr "Example 2"
	jgraph -P example2.jgr | ps2pdf - | convert -density 300 - -quality 100 example2.jpg
three:
	./happyQR --output example3.jgr "Example 3: Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean imperdiet. Etiam ultricies nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus, tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. Nam quam nunc, blandit vel, luctus pulvinar, hendrerit id, lorem. Maecenas nec odio et ante tincidunt tempus. Donec vitae sapien ut libero venenatis faucibus. Nullam quis ante. Etiam sit amet orci eget eros faucibus tincidunt. Duis leo. Sed fringilla mauris sit amet nibh. Donec sodales sagittis magna. Sed consequat, leo eget bibendum sodales, augue velit cursus nunc, quis gravida magna mi a libero. Fusce vulputate eleifend sapien. Vestibulum purus quam, scelerisque ut, mollis sed, nonummy id, metus. Nullam accumsan lorem in dui. Cras ultricies mi eu turpis hendrerit fringilla. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; In ac dui quis mi consectetuer lacinia. Nam pretium turpis et arcu. Duis arcu tortor, suscipit eget, imperdiet nec, imperdiet iaculis, ipsum. Sed aliquam ultrices mauris. Integer ante arcu, accumsan a, consectetuer eget, posuere ut, mauris. Praesent adipiscing. Phasellus ullamcorper ipsum rutrum nunc. Nunc nonummy metus. Vestibulum volutpat pretium libero. Cras id dui. Aenean ut eros et nisl sagittis vestibulum. Nullam nulla eros, ultricies sit amet, nonummy id, imperdiet feugiat, pede. Sed lectus. Donec mollis hendrerit risus. Phasellus nec sem in justo pellentesque facilisis. Etiam imperdiet imperdiet orci. Nunc nec neque. Phasellus leo dolor, tempus non, auctor et, hendrerit quis, nisi. Curabitur ligula sapien, tincidunt non, euismod vitae, posuere imperdiet, leo. Maecenas malesuada. Praesent congue erat at massa. Sed cursus turpis vitae tortor. Donec posuere vulputate arcu. Phasellus accumsan cursus velit. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed aliquam, nisi quis porttitor congue, elit erat euismod orci, ac"
	jgraph -P example3.jgr | ps2pdf - | convert -density 300 - -quality 100 example3.jpg
four:
	./happyQR --light pat 100 "linethickness 0.1 pcfill 1.0 0.8 1.0 ppattern estripe 75" --dark pat 100 "pcfill 0.2 0 0.2 ppattern estripe 75" --output example4.jgr "Example 4?"
	jgraph -P example4.jgr | ps2pdf - | convert -density 300 - -quality 100 example4.jpg
five:
	./happyQR --dark img 100 blackFox.jpg --light img 100 bulldog.jpg --output example5.jgr "Example 5"
	jgraph -P example5.jgr | ps2pdf - | convert -density 300 - -quality 100 example5.jpg
