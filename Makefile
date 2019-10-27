fft: fft.cpp util.cpp driver.cpp
	g++ fft.cpp util.cpp driver.cpp -o fft
run: fft
	./fft [1,2,3]
clean:
	rm fft