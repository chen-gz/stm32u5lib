## note of stm32u5 usb otg hs

### 1. control endpoint 0 

- When using dma, the transfer complete interrupt will not clear the `epena` bit in `doepctl` register;
- Although `dmaen` is been set, if the `transfer size` is 0 in the `doeptsiz` register, teh dma will not be triggered;


I was tried to use endpoint 0 with dma, but I give up finally. 
I found that the dma may not work as documented (rm0456).


### when we not using dma for out endpoint,

The in dndpint will not give the transfer complete interrupt although the transfer comple


* the endpoints will not reset after "reset" signal, I think it should be reset by the hardware since usbaep is been reset.
* current solutions is just leave the nak unchanged
* 