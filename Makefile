fft: fft.cpp util.cpp main.cpp
	g++ fft.cpp util.cpp main.cpp -o fft
run: fft
	./fft [1,2,3]
clean:
	rm fft