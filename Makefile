fft: fft.cpp util.cpp driver.cpp
	g++ fft.cpp util.cpp driver.cpp -o fft
run: fft
	./fft [0,0.707,1,0.707,0,-0.707,-1,-0.707]
clean:
	rm fft