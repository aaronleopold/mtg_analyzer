from django.shortcuts import render,redirect
from django.views.generic import TemplateView
from django.http import HttpResponseRedirect
from mtgsdk import Card



def dev(request):
    cards = Card.where(page=5).where(pageSize=10).all()
    cards = list(filter(lambda card: card.image_url is not None, cards))  

    context = {
        'cards': cards
    }
    return render(request, 'cards.html', context)