# note of stm32u5 usb otg hs

I was tried to use endpoint 0 with dma, but I give up finally. I found that the
dma may not work as documented (rm0456).


### when we not using dma for out endpoint,

The in dndpint will not give the transfer complete interrupt although the
transfer comple


* the endpoints will not reset after "reset" signal, I think it should be reset
by the hardware since usbaep is been reset.
* current solutions is just leave the nak unchanged

### windows does not following the usb standard

the setup packet may not have status stage. So we should not 



## this is notes for stm32u5 usb otg hs

A lot of implementation does not clear and not memtion in the reference manual.

Please read this before do any modification to this library.


### DMA

The `dma` is required when the `usb otg hs` is working in `high speed`. If using
the interreupt mode, the `usb otg hs` can not get very high speed. Please
considierusing `otg_fs` library when dma does not applied to the `usb otg hs`.


### Control Point 

- When using dma, the transfer complete interrupt will not clear the `epena` bit
in `doepctl` register; (this is different than the statment in the reference
manual)
- Although `dmaen` is been set, if the `transfer size` is 0 in the `doeptsiz`
register, the dma will not be triggered; The transfer size should be set
accordingly. 
- The `rxflvl` interrupt should be not used. When I develop this library, I am
not able to make it work with the `rxflvl` interrupt. Either `rxflvl` or `epout`
will work.

** Anyway, be careful when update this library, although current code in this
library is ugly, but it works. **
