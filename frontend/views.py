from django.shortcuts import render,redirect
from django.views.generic import TemplateView
from django.http import HttpResponseRedirect
from django.db.models import Q
from mtgsdk import Card



def dev(request):
    cards = Card.where(page=5).where(pageSize=10).all()
    # cards = Card.where(page=5).where(pageSize=10).where(~Q(image_url=None)).all()
    cards = list(filter(lambda card: card.image_url is not None, cards))

    # print(type(cards[3].loyalty))
    # print(cards[3].loyalty)

    context = {
        'cards': cards
    }
    return render(request, 'cards.html', context)